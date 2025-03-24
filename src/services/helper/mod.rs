use crate::Failure;

use super::{
    config::codes::ResultCode,
    crypt::CryptService,
    db::{
        records::{CommandUsageRecord, Record, RecordKey, ToolsAnalytics},
        DBService,
    },
};

pub struct HelperService {
    pub crypt: CryptService,
    pub db: DBService,
}

impl HelperService {
    pub const fn new(crypt: CryptService, db: DBService) -> Self {
        HelperService { crypt, db }
    }

    pub fn generate_inquire_error(&self, e: inquire::InquireError) -> String {
        match e {
            inquire::InquireError::NotTTY =>
                "Input device is not a TTY".to_string(),
            inquire::InquireError::InvalidConfiguration(reason) =>
                format!("Invalid configuration: {}", reason),
            inquire::InquireError::IO(io_err) => format!("IO error: {}", io_err),
            inquire::InquireError::OperationCanceled =>
                "Operation canceled by the user".to_string(),
            inquire::InquireError::OperationInterrupted =>
                "Operation interrupted by the user".to_string(),
            inquire::InquireError::Custom(custom_err) =>
                format!("Custom error: {}", custom_err),
        }
    }

    pub fn update_command_usage(&self, tool: ToolsAnalytics) -> Result<(), Failure> {
        let mut analytics = self.db.get_record(&RecordKey::CommandUsage)?;

        if analytics.is_none() {
            analytics = Some(Record::new(
                RecordKey::CommandUsage,
                serde_json::to_value(CommandUsageRecord::default()).unwrap(),
                vec![],
            ));
        }

        if let Some(analytics) = analytics {
            let value = analytics.value_as::<CommandUsageRecord>();
            match value {
                Ok(mut record) => {
                    let mut found = false;
                    for (key, value) in record.values.iter_mut() {
                        if *key == tool.value() {
                            *value += 1;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        record.values.push((tool.value().to_string(), 1));
                    }

                    self.db.create_or_update_record(&Record::new(
                        RecordKey::CommandUsage,
                        serde_json::to_value(record).unwrap(),
                        vec![],
                    ))?;
                }
                _ => {}
            }
        };

        Ok(())
    }

    pub fn reset_command_usage(&self) -> Result<(), Failure> {
        self.db.create_or_update_record(&Record::new(
            RecordKey::CommandUsage,
            serde_json::to_value(CommandUsageRecord::default()).unwrap(),
            vec![],
        ))?;

        Ok(())
    }

    pub fn get_command_usage_data(&self) -> Result<Vec<(String, i32)>, Failure> {
        let mut analytics = self.db.get_record(&RecordKey::CommandUsage)?;

        if analytics.is_none() {
            analytics = Some(Record::new(
                RecordKey::CommandUsage,
                serde_json::to_value(CommandUsageRecord::default()).unwrap(),
                vec![],
            ));
        }

        if let Some(analytics) = analytics {
            self.db.create_or_update_record(&analytics)?;

            let value = analytics.value_as::<CommandUsageRecord>();
            match value {
                Ok(record) => Ok(record.values),
                _ => Err(Failure {
                    message: "Invalid analytics data".to_string(),
                    trace: "".to_string(),
                    code: ResultCode::AnalyticsFetchFailed,
                }),
            }
        } else {
            Err(Failure {
                message: "Invalid analytics data".to_string(),
                trace: "".to_string(),
                code: ResultCode::AnalyticsFetchFailed,
            })
        }
    }
}
