use std::error::Error;
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread::{sleep, Thread};
use std::time::Duration;

#[derive(Debug)]
pub enum eBMOpErrorLevel {
  BMO_ERROR_CANCEL = 0,
  BMO_ERROR_WARN = 1,
  BMO_ERROR_FATAL = 2,
}

pub enum BMOpError {
  BMO_ERROR_INVALID_INPUT,
  BMO_ERROR_UNKNOWN_OPERATION,
  BMO_ERROR_INVALID_MESH,
  BMO_ERROR_INVALID_PARAMETERS,
  BMO_ERROR_INVALID_DATA,
  BMO_ERROR_INTERNAL_FAILURE,
}

pub struct BMOpErrorMessage {
  code: eBMOpErrorLevel,
  message: String,
}

pub struct BMOpErrorStack {
  top: BMOpErrorMessage,
  mutex: Mutex,
}

pub fn BMO_error_raise(bm: &mut BMesh, owner: &mut BMOperator, level: eBMOpErrorLevel, msg: String) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: level,
      message: msg,
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_push(&mut stack, &owner, &level, &msg);

  return Ok(());
}

pub fn BMO_error_get(bm: &mut BMesh, r_msg: &mut String, r_op: &mut BMOperator, r_level: &mut eBMOpErrorLevel) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: eBMOpErrorLevel::BMO_ERROR_WARN,
      message: "Error occurred, but the mesh can still be used.",
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_pop(&mut stack, &r_msg, &r_op, &r_level);

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  return Ok(());
}

pub fn BMO_error_get_at_level(bm: &mut BMesh, level: eBMOpErrorLevel, r_msg: &mut String, r_op: &mut BMOperator) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: level,
      message: "Error occurred.",
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_pop(&mut stack, &r_msg, &r_op, &level);

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  return Ok(());
}

pub fn BMO_error_occurred_at_level(bm: &mut BMesh, level: eBMOpErrorLevel) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: level,
      message: "Error occurred.",
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_pop(&mut stack, &_, &_, level);

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  return Ok(());
}

pub fn BMO_error_pop(bm: &mut BMesh, r_msg: &mut String, r_op: &mut BMOperator, r_level: &mut eBMOpErrorLevel) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: eBMOpErrorLevel::BMO_ERROR_WARN,
      message: "Error occurred, but the mesh can still be used.",
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_pop(&mut stack, &r_msg, &r_op, &r_level);

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  return Ok(());
}

pub fn BMO_error_clear(bm: &mut BMesh) -> Result<(), BMOpError> {
  let mut stack = BMOpErrorStack {
    top: BMOpErrorMessage {
      code: eBMOpErrorLevel::BMO_ERROR_WARN,
      message: "Error occurred, but the mesh can still be used.",
    },
    mutex: Mutex::new(),
  };

  bm_error_stack_clear(&mut stack);

  return Ok(());
}

pub fn BMOpErrorMessage::from_error(error: BMOpError) -> Self {
  Self {
    code: error.code,
    message: error.message,
  }
}

pub fn BMOpErrorStack::new() -> Self {
  Self {
    top: BMOpErrorMessage::from_error(BMOpError::BMO_ERROR_WARN),
    mutex: Mutex::new(),
  }
}

pub fn BMOpErrorStack::push(&mut self, owner: &mut BMOperator, level: eBMOpErrorLevel, msg: String) -> Result<(), BMOpError> {
  let mut stack = self;

  self.mutex.lock();
  defer { self.mutex.unlock(); }

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  stack.top = BMOpErrorMessage::from_error(BMOpError::BMO_ERROR_INVALID_INPUT, msg);

  return Ok(());
}

pub fn BMOpErrorStack::pop(&mut self, r_msg: &mut String, r_op: &mut BMOperator, r_level: &mut eBMOpErrorLevel) -> Result<(), BMOpError> {
  let mut stack = self;

  self.mutex.lock();
  defer { self.mutex.unlock(); }

  if let Some(error) = stack.top.message {
    return Err(BMOpError::BMO_ERROR_INVALID_INPUT, error);
  }

  stack.top = BMOpErrorMessage::from_error(BMOpError::BMO_ERROR_WARN);

  r_msg = stack.top.message;
  r_op = stack.top.message.owner;
  r_level = stack.top.message.code;

  return Ok(());
}

pub fn BMOpErrorStack::clear(&mut self) -> Result<(), BMOpError> {
  let mut stack = self;

  self.mutex.lock();
  defer { self.mutex.unlock(); }

  stack.top = BMOpErrorMessage::from_error(BMOpError::BMO_ERROR_WARN);

  return Ok(());
}

pub fn bm_error
