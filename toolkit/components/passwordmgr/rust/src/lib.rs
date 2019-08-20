#![allow(non_snake_case)]

use std::{cell::RefCell, path::PathBuf};

#[macro_use]
extern crate xpcom;

#[macro_use]
extern crate cstr;

use nserror::{nsresult, NS_OK, NS_ERROR_NOT_IMPLEMENTED};
use nsstring::{nsAString, nsString};
use thin_vec::ThinVec;
use xpcom::{
    interfaces::{
        nsILoginManagerBase, nsILoginInfo, nsISupports, nsIPropertyBag, nsIFile,
    },
    RefPtr, XpCom,
};
use logins::{PasswordEngine, Error};

#[no_mangle]
pub unsafe extern "C" fn NS_NewRustLoginManager(
    result: *mut *const nsILoginManagerBase,
) {
    let manager = LoginManager::new();
    RefPtr::new(manager.coerce::<nsILoginManagerBase>()).forget(&mut *result);
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
        profile_dir.GetPath(&mut *profile_path).to_result().expect("Can't get profile directory path");
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
    fn add_login(&self, login: &nsILoginInfo) -> Result<RefPtr<nsILoginInfo>, nsresult> {
        println!("Adding login in Rust...");
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(remove_login => RemoveLogin(login: *const nsILoginInfo));
    fn remove_login(&self, login: &nsILoginInfo) -> Result<(), nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(modify_login => ModifyLogin(old_login: *const nsILoginInfo, new_login_data: *const nsISupports));
    fn modify_login(&self, old_login: &nsILoginInfo, new_login_data: &nsISupports) -> Result<(), nsresult> {
        if let Some(login) = new_login_data.query_interface::<nsILoginInfo>() {
            println!("Modifying login with new `nsILoginInfo`");
        } else if let Some(info) = new_login_data.query_interface::<nsIPropertyBag>() {
            println!("Modifying login with new properties");
        }
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(remove_all_logins => RemoveAllLogins());
    fn remove_all_logins(&self) -> Result<(), nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
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
    fn find_logins(&self, hostname: &nsAString, action_url: Option<&nsAString>, http_realm: Option<&nsAString>) -> Result<ThinVec<RefPtr<nsILoginInfo>>, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(count_logins => CountLogins(hostname: *const nsAString, action_url: *const nsAString, http_realm: *const nsAString) -> u32);
    fn count_logins(&self, hostname: &nsAString, action_url: Option<&nsAString>, http_realm: Option<&nsAString>) -> Result<u32, nsresult> {
        Err(NS_ERROR_NOT_IMPLEMENTED)
    }

    xpcom_method!(search_logins => SearchLogins(match_data: *const nsIPropertyBag) -> ThinVec<RefPtr<nsILoginInfo>>);
    fn search_logins(&self, match_data: &nsIPropertyBag) -> Result<ThinVec<RefPtr<nsILoginInfo>>, nsresult> {
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
