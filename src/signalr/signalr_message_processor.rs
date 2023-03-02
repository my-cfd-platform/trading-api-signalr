use std::sync::Arc;

use crate::AppContext;

pub struct SignalRMessageProcessor{
    app_context: Arc<AppContext>,
}