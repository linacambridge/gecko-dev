const { nsLoginInfo } = ChromeUtils.import(
  "resource://gre/modules/LoginInfo.jsm"
);

add_task(async function test_rust_logins() {
  const manager = Cc["@mozilla.org/login-manager/rust;1"].getService(
    Ci.nsILoginManagerBase
  );
  let loginInfo = new nsLoginInfo();
  loginInfo.init(
    "http://example.com",
    "http://example.com/action",
    null,
    "user",
    "s3kr1t"
  );
  let newLogin = manager.addLogin(loginInfo);
  info(`Added new login: ${JSON.stringify(newLogin)}`);
});
