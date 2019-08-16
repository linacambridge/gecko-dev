#ifndef mozilla_passwordmgr_LoginManager_h_
#define mozilla_passwordmgr_LoginManager_h_

#include "nsCOMPtr.h"
#include "nsILoginManager.h"

extern "C" {

// Implemented in Rust.
void NS_NewRustLoginManager(nsILoginManagerBase** aResult);

}  // extern "C"

namespace mozilla {

already_AddRefed<nsILoginManagerBase> NewRustLoginManager() {
  nsCOMPtr<nsILoginManagerBase> manager;
  NS_NewRustLoginManager(getter_AddRefs(manager));
  return manager.forget();
}

}  // namespace mozilla

#endif  // mozilla_passwordmgr_LoginManager_h_
