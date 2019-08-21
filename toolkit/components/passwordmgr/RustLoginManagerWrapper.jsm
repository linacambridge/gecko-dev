const { Services } = ChromeUtils.import("resource://gre/modules/Services.jsm");

/// A wrapper around the Rust-implemented `nsILoginManagerBase`. This exists
/// because Rust XPCOM doesn't support `jsval`s (including promises), so
/// we need to implement the extra methods on `nsILoginManager` in JS
/// (or C++, but why? 😱)

function RustLoginManagerWrapper() {
  this.initializationPromise = Promise.resolve();
  this._backend = Cc["@mozilla.org/login-manager/rust;1"].getService(
    Ci.nsILoginManagerBase
  );
}

RustLoginManagerWrapper.prototype = {
  classID: Components.ID("{cb9e0de8-3598-4ed7-857b-827f011ad5d8}"),
  QueryInterface: ChromeUtils.generateQI([
    Ci.nsILoginManagerBase,
    Ci.nsILoginManager,
    Ci.nsISupportsWeakReference,
  ]),

  addLogin(login) {
    return this._backend.addLogin(login);
  },

  removeAllLogins() {
    this._backend.removeAllLogins();
  },

  getAllDisabledHosts() {
    return [];
  },

  async addLogins(logins) {
    throw new Error("Not implemented");
  },

  async getAllLoginsAsync() {
    throw new Error("Not implemented");
  },
};

const EXPORTED_SYMBOLS = ["RustLoginManagerWrapper"];
