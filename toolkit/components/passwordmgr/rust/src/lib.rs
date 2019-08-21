#![allow(non_snake_case)]

use std::{cell::RefCell, path::PathBuf};

#[macro_use]
extern crate xpcom;

#[macro_use]
extern crate cstr;

use logins::{Error, Login, PasswordEngine};
use nserror::{nsresult, NS_ERROR_NOT_IMPLEMENTED, NS_OK, NS_ERROR_FAILURE};
use nsstring::{nsAString, nsString};
use thin_vec::ThinVec;
use uuid::Uuid;
use xpcom::{
    interfaces::{
        nsIFile, nsILoginInfo, nsILoginManagerBase, nsILoginMetaInfo, nsIPropertyBag, nsISupports,
    },
    RefPtr, XpCom,
};

#[no_mangle]
pub unsafe extern "C" fn NS_NewRustLoginManager(result: *mut *const nsILoginManagerBase) {
    let manager = LoginManager::new();
    RefPtr::new(manager.coerce::<nsILoginManagerBase>()).forget(&mut *result);
}

#[derive(xpcom)]
#[xpimplements(nsILoginInfo, nsILoginMetaInfo)]
#[refcnt = "nonatomic"]
struct InitRawLogin {
    login: Login,
}

// `nsILoginInfo` methods.
impl RawLogin {
    xpcom_method!(init => Init(aOrigin: *const nsAString, aFormActionOrigin: *const nsAString, aHttpRealm: *const nsAString, aUsername: *const nsAString, aPassword: *const nsAString, aUsernameField: *const nsAString, aPasswordField: *const nsAString));
    fn init(&self, origin: &nsAString, form_action_origin: Option<&nsAString>, http_realm: Option<&nsAString>, username: Option<&nsAString>, password: Option<&nsAString>, username_field: Option<&nsAString>, password_field: Option<&nsAString>) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_origin => GetOrigin() -> nsAString);
    fn get_origin(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.hostname))
    }

    xpcom_method!(set_origin => SetOrigin(origin: *const nsAString));
    fn set_origin(&self, origin: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_hostname => GetHostname() -> nsAString);
    fn get_hostname(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.hostname))
    }

    xpcom_method!(set_hostname => SetHostname(hostname: *const nsAString));
    fn set_hostname(&self, hostname: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_form_action_origin => GetFormActionOrigin() -> nsAString);
    fn get_form_action_origin(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.form_submit_url.clone().unwrap_or_default()))
    }

    xpcom_method!(set_form_action_origin => SetFormActionOrigin(origin: *const nsAString));
    fn set_form_action_origin(&self, origin: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_form_submit_url => GetFormSubmitURL() -> nsAString);
    fn get_form_submit_url(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.form_submit_url.clone().unwrap_or_default()))
    }

    xpcom_method!(set_form_submit_url => SetFormSubmitURL(origin: *const nsAString));
    fn set_form_submit_url(&self, form_submit_url: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_http_realm => GetHttpRealm() -> nsAString);
    fn get_http_realm(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.http_realm.clone().unwrap_or_default()))
    }

    xpcom_method!(set_http_realm => SetHttpRealm(http_realm: *const nsAString));
    fn set_http_realm(&self, http_realm: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_username => GetUsername() -> nsAString);
    fn get_username(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.username))
    }

    xpcom_method!(set_username => SetUsername(origin: *const nsAString));
    fn set_username(&self, username: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_username_field => GetUsernameField() -> nsAString);
    fn get_username_field(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.username_field))
    }

    xpcom_method!(set_username_field => SetUsernameField(origin: *const nsAString));
    fn set_username_field(&self, username_field: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_password => GetPassword() -> nsAString);
    fn get_password(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.password))
    }

    xpcom_method!(set_password => SetPassword(origin: *const nsAString));
    fn set_password(&self, password: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_password_field => GetPasswordField() -> nsAString);
    fn get_password_field(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.password_field))
    }

    xpcom_method!(set_password_field => SetPasswordField(origin: *const nsAString));
    fn set_password_field(&self, password_field: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(equals => Equals(other: *const nsILoginInfo) -> bool);
    fn equals(&self, other: &nsILoginInfo) -> Result<bool, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(matches => Matches(aLoginInfo: *const nsILoginInfo, ignorePassword: bool) -> bool);
    fn matches(&self, login_info: &nsILoginInfo, ignore_password: bool) -> Result<bool, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(clone => Clone() -> *const nsILoginInfo);
    fn clone(&self) -> Result<RefPtr<nsILoginInfo>, nsresult> {
        Ok(RefPtr::new(RawLogin::allocate(InitRawLogin {
            login: self.login.clone()
        }).coerce::<nsILoginInfo>()))
    }
}

// `nsILoginMetaInfo` methods.
impl RawLogin {
    xpcom_method!(get_origin => GetGuid() -> nsAString);
    fn get_guid(&self) -> Result<nsString, nsresult> {
        Ok(nsString::from(&*self.login.guid.as_str()))
    }

    xpcom_method!(set_origin => SetGuid(guid: *const nsAString));
    fn set_guid(&self, guid: &nsAString) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_time_created => GetTimeCreated() -> u64);
    fn get_time_created(&self) -> Result<u64, nsresult> {
        Ok(self.login.time_created as u64)
    }

    xpcom_method!(set_time_created => SetTimeCreated(time_created: u64));
    fn set_time_created(&self, time_created: u64) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_time_last_used => GetTimeLastUsed() -> u64);
    fn get_time_last_used(&self) -> Result<u64, nsresult> {
        Ok(self.login.time_last_used as u64)
    }

    xpcom_method!(set_time_last_used => SetTimeLastUsed(time: u64));
    fn set_time_last_used(&self, time: u64) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_time_password_changed => GetTimePasswordChanged() -> u64);
    fn get_time_password_changed(&self) -> Result<u64, nsresult> {
        Ok(self.login.time_password_changed as u64)
    }

    xpcom_method!(set_time_password_changed => SetTimePasswordChanged(time: u64));
    fn set_time_password_changed(&self, time: u64) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }

    xpcom_method!(get_times_used => GetTimesUsed() -> u32);
    fn get_times_used(&self) -> Result<u32, nsresult> {
        Ok(self.login.times_used as u32)
    }

    xpcom_method!(set_times_used => SetTimesUsed(times: u32));
    fn set_times_used(&self, times: u32) -> Result<(), nsresult> {
        Err(NS_ERROR_FAILURE)
    }
}

#[derive(Default)]
struct LoginMeta {
    guid: String,
    time_created: i64,
    time_last_used: i64,
    time_password_changed: i64,
    times_used: i64,
}

impl LoginMeta {
    fn new() -> LoginMeta {
        let guid = Uuid::new_v4().hyphenated().to_string();
        LoginMeta {
            guid,
            ..Default::default()
        }
    }
}

impl From<&nsILoginMetaInfo> for LoginMeta {
    fn from(meta: &nsILoginMetaInfo) -> LoginMeta {
        let mut raw_guid = nsString::new();
        unsafe { meta.GetGuid(&mut *raw_guid) }
            .to_result()
            .expect("oh no its empty 😬");
        let guid = if raw_guid.is_empty() {
            Uuid::new_v4().hyphenated().to_string()
        } else {
            String::from_utf16(&*raw_guid).expect("What kind of GUID are you passing? 😳")
        };
        LoginMeta {
            guid,
            ..Default::default()
        }
    }
}

#[derive(xpcom)]
#[xpimplements(nsILoginManagerBase)]
#[refcnt = "nonatomic"]
pub struct InitLoginManager {
    engine: PasswordEngine,
}

fn get_profile_dir() -> PathBuf {
    // We can't use getter_addrefs() here because get_DirectoryService()
    // returns its nsIProperties interface, and its Get() method returns
    // a directory via its nsQIResult out param, which gets translated to
    // a `*mut *mut libc::c_void` in Rust, whereas getter_addrefs() expects
    // a closure with a `*mut *const T` parameter.

    let dir_svc = xpcom::services::get_DirectoryService().expect("Can't get directory service");
    let mut profile_dir = xpcom::GetterAddrefs::<nsIFile>::new();
    unsafe {
        dir_svc
            .Get(
                cstr!("ProfD").as_ptr(),
                &nsIFile::IID,
                profile_dir.void_ptr(),
            )
            .to_result()
            .or_else(|_| {
                dir_svc
                    .Get(
                        cstr!("ProfDS").as_ptr(),
                        &nsIFile::IID,
                        profile_dir.void_ptr(),
                    )
                    .to_result()
            })
            .expect("Can't get profile dir");
    }
    let profile_dir = profile_dir.refptr().expect("Can't QI to `nsIFile`");

    let mut profile_path = nsString::new();
    unsafe {
        profile_dir
            .GetPath(&mut *profile_path)
            .to_result()
            .expect("Can't get profile directory path");
    }

    let path = String::from_utf16(&profile_path[..]).expect("Can't convert profile path to string");
    PathBuf::from(&path)
}

impl LoginManager {
    pub fn new() -> RefPtr<LoginManager> {
        let mut path = get_profile_dir();
        path.push("logins2.sqlite");
        LoginManager::allocate(InitLoginManager {
            engine: PasswordEngine::new(path, None).expect("Failed to open DB"),
        })
    }

    xpcom_method!(add_login => AddLogin(login: *const nsILoginInfo) -> *const nsILoginInfo);
    fn add_login(&self, new_login: &nsILoginInfo) -> Result<RefPtr<nsILoginInfo>, nsresult> {
        println!("Adding login in Rust...");
        let meta = if let Some(meta) = new_login.query_interface::<nsILoginMetaInfo>() {
            LoginMeta::from(&*meta)
        } else {
            LoginMeta::new()
        };
        let mut hostname = nsString::new();
        unsafe { new_login.GetHostname(&mut *hostname) }
            .to_result()
            .expect("No hostname 🙊");
        let mut password = nsString::new();
        unsafe { new_login.GetPassword(&mut *password) }
            .to_result()
            .expect("No password 🙊");
        let login = Login {
            guid: meta.guid.into(),
            hostname: String::from_utf16(&*hostname)
                .expect("What kind of hostname are you passing? 😳"),
            form_submit_url: None,
            http_realm: Some("foobar".to_string()),
            username: String::new(),
            password: String::from_utf16(&*hostname)
                .expect("What kind of password are you passing? 😳"),
            username_field: String::new(),
            password_field: String::new(),
            time_created: 0i64,
            time_password_changed: 0i64,
            time_last_used: 0i64,
            times_used: 0,
        };
        let new_guid = self.engine.add(login).expect("Failed to add login! 😱");
        let added_login = self.engine.get(&new_guid).expect("Failed to fetch login 😭").expect("The login we literally just added doesn't exist 🤪");
        Ok(RefPtr::new(RawLogin::allocate(InitRawLogin {
            login: added_login,
        }).coerce::<nsILoginInfo>()))
    }

    xpcom_method!(remove_login => RemoveLogin(login: *const nsILoginInfo));
    fn remove_login(&self, login: &nsILoginInfo) -> Result<(), nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(modify_login => ModifyLogin(old_login: *const nsILoginInfo, new_login_data: *const nsISupports));
    fn modify_login(
        &self,
        old_login: &nsILoginInfo,
        new_login_data: &nsISupports,
    ) -> Result<(), nsresult> {
        if let Some(login) = new_login_data.query_interface::<nsILoginInfo>() {
            println!("Modifying login with new `nsILoginInfo`");
        } else if let Some(info) = new_login_data.query_interface::<nsIPropertyBag>() {
            println!("Modifying login with new properties");
        }
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(remove_all_logins => RemoveAllLogins());
    fn remove_all_logins(&self) -> Result<(), nsresult> {
        self.engine.wipe().expect("Failed to wipe 🚿");
        Ok(())
    }

    xpcom_method!(get_all_logins => GetAllLogins() -> ThinVec<RefPtr<nsILoginInfo>>);
    fn get_all_logins(&self) -> Result<ThinVec<RefPtr<nsILoginInfo>>, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(get_all_disabled_hosts => GetAllDisabledHosts() -> ThinVec<nsString>);
    fn get_all_disabled_hosts(&self) -> Result<ThinVec<nsString>, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(get_login_saving_enabled => GetLoginSavingEnabled(host: *const nsAString) -> bool);
    fn get_login_saving_enabled(&self, host: &nsAString) -> Result<bool, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(set_login_saving_enabled => SetLoginSavingEnabled(host: *const nsAString, enabled: bool));
    fn set_login_saving_enabled(&self, host: &nsAString, enabled: bool) -> Result<(), nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(find_logins => FindLogins(hostname: *const nsAString, action_url: *const nsAString, http_realm: *const nsAString) -> ThinVec<RefPtr<nsILoginInfo>>);
    fn find_logins(
        &self,
        hostname: &nsAString,
        action_url: Option<&nsAString>,
        http_realm: Option<&nsAString>,
    ) -> Result<ThinVec<RefPtr<nsILoginInfo>>, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(count_logins => CountLogins(hostname: *const nsAString, action_url: *const nsAString, http_realm: *const nsAString) -> u32);
    fn count_logins(
        &self,
        hostname: &nsAString,
        action_url: Option<&nsAString>,
        http_realm: Option<&nsAString>,
    ) -> Result<u32, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(search_logins => SearchLogins(match_data: *const nsIPropertyBag) -> ThinVec<RefPtr<nsILoginInfo>>);
    fn search_logins(
        &self,
        match_data: &nsIPropertyBag,
    ) -> Result<ThinVec<RefPtr<nsILoginInfo>>, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(get_ui_busy => GetUiBusy() -> bool);
    fn get_ui_busy(&self) -> Result<bool, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(get_is_logged_in => GetIsLoggedIn() -> bool);
    fn get_is_logged_in(&self) -> Result<bool, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }
}
