/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use failure::Fail;

// TODO: this is (IMO) useful and was dropped from `failure`, consider moving it
// into `error_support`.
macro_rules! throw {
    ($e:expr) => {
        return Err(Into::into($e));
    };
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Invalid login: {}", _0)]
    InvalidLogin(InvalidLogin),

    #[fail(
        display = "The `sync_status` column in DB has an illegal value: {}",
        _0
    )]
    BadSyncStatus(u8),

    #[fail(display = "A duplicate GUID is present: {:?}", _0)]
    DuplicateGuid(String),

    #[fail(
        display = "No record with guid exists (when one was required): {:?}",
        _0
    )]
    NoSuchRecord(String),

    #[fail(display = "Error synchronizing: {}", _0)]
    SyncAdapterError(#[fail(cause)] sync15::Error),

    #[fail(display = "Error parsing JSON data: {}", _0)]
    JsonError(#[fail(cause)] serde_json::Error),

    #[fail(display = "Error executing SQL: {}", _0)]
    SqlError(#[fail(cause)] rusqlite::Error),

    #[fail(display = "Error parsing URL: {}", _0)]
    UrlParseError(#[fail(cause)] url::ParseError),

    #[fail(display = "{}", _0)]
    Interrupted(#[fail(cause)] interrupt::Interrupted),
}

error_support::define_error! {
    ErrorKind {
        (SyncAdapterError, sync15::Error),
        (JsonError, serde_json::Error),
        (UrlParseError, url::ParseError),
        (SqlError, rusqlite::Error),
        (InvalidLogin, InvalidLogin),
        (Interrupted, interrupt::Interrupted),
    }
}

#[derive(Debug, Fail)]
pub enum InvalidLogin {
    #[fail(display = "Hostname is empty")]
    EmptyHostname,
    #[fail(display = "Password is empty")]
    EmptyPassword,
    #[fail(display = "Both `formSubmitUrl` and `httpRealm` are present")]
    BothTargets,
    #[fail(display = "Neither `formSubmitUrl` and `httpRealm` are present")]
    NoTarget,
}
