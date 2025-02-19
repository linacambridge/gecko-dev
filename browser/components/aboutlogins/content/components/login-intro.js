/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

export default class LoginIntro extends HTMLElement {
  connectedCallback() {
    if (this.shadowRoot) {
      return;
    }

    let loginIntroTemplate = document.querySelector("#login-intro-template");
    let shadowRoot = this.attachShadow({ mode: "open" });
    document.l10n.connectRoot(shadowRoot);
    shadowRoot.appendChild(loginIntroTemplate.content.cloneNode(true));

    shadowRoot.addEventListener("click", this);
  }

  handleEvent(event) {
    let faqLink = this.shadowRoot.querySelector(".intro-faq-link");

    if (event.type == "click" && event.originalTarget == faqLink) {
      document.dispatchEvent(
        new CustomEvent("AboutLoginsOpenFAQ", {
          bubbles: true,
        })
      );
    }
  }
}
customElements.define("login-intro", LoginIntro);
