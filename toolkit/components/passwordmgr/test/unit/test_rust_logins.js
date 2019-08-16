const { nsLoginInfo } = ChromeUtils.import(
  "resource://gre/modules/LoginInfo.jsm"
);

add_task(async function test_rust_logins() {
  const manager = Cc["@mozilla.org/login-manager/rust;1"].getService(
    Ci.nsILoginManagerBase
  );
  let info = new nsLoginInfo();
  info.init(
    "http://example.com",
    "http://example.com/action",
    null,
    "user",
    "s3kr1t"
  );
  manager.addLogin(info);
});
