#include "cert.h"
#include "certdb.h"
#include "nspr.h"
#include "nss.h"
#include "pk11pub.h"
#include "secmod.h"
#include "secerr.h"

#include "nss_scoped_ptrs.h"
#include "util.h"

#define GTEST_HAS_RTTI 0
#include "gtest/gtest.h"

namespace nss_test {

class SoftokenTest : public ::testing::Test {
 protected:
  SoftokenTest() : mNSSDBDir("SoftokenTest.d-") {}
  SoftokenTest(const std::string &prefix) : mNSSDBDir(prefix) {}

  virtual void SetUp() {
    std::string nssInitArg("sql:");
    nssInitArg.append(mNSSDBDir.GetUTF8Path());
    ASSERT_EQ(SECSuccess, NSS_Initialize(nssInitArg.c_str(), "", "", SECMOD_DB,
                                         NSS_INIT_NOROOTINIT));
  }

  virtual void TearDown() {
    ASSERT_EQ(SECSuccess, NSS_Shutdown());
    const std::string &nssDBDirPath = mNSSDBDir.GetPath();
    ASSERT_EQ(0, unlink((nssDBDirPath + "/cert9.db").c_str()));
    ASSERT_EQ(0, unlink((nssDBDirPath + "/key4.db").c_str()));
    ASSERT_EQ(0, unlink((nssDBDirPath + "/pkcs11.txt").c_str()));
  }

  ScopedUniqueDirectory mNSSDBDir;
};

TEST_F(SoftokenTest, ResetSoftokenEmptyPassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
  EXPECT_EQ(SECSuccess, PK11_ResetToken(slot.get(), nullptr));
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
}

TEST_F(SoftokenTest, ResetSoftokenNonEmptyPassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, "password"));
  EXPECT_EQ(SECSuccess, PK11_ResetToken(slot.get(), nullptr));
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, "password2"));
}

// Test certificate to use in the CreateObject tests.
static const CK_OBJECT_CLASS cko_nss_trust = CKO_NSS_TRUST;
static const CK_BBOOL ck_false = CK_FALSE;
static const CK_BBOOL ck_true = CK_TRUE;
static const CK_TRUST ckt_nss_must_verify_trust = CKT_NSS_MUST_VERIFY_TRUST;
static const CK_TRUST ckt_nss_trusted_delegator = CKT_NSS_TRUSTED_DELEGATOR;
static const CK_ATTRIBUTE attributes[] = {
    {CKA_CLASS, (void *)&cko_nss_trust, (PRUint32)sizeof(CK_OBJECT_CLASS)},
    {CKA_TOKEN, (void *)&ck_true, (PRUint32)sizeof(CK_BBOOL)},
    {CKA_PRIVATE, (void *)&ck_false, (PRUint32)sizeof(CK_BBOOL)},
    {CKA_MODIFIABLE, (void *)&ck_false, (PRUint32)sizeof(CK_BBOOL)},
    {CKA_LABEL,
     (void *)"Symantec Class 2 Public Primary Certification Authority - G4",
     (PRUint32)61},
    {CKA_CERT_SHA1_HASH,
     (void *)"\147\044\220\056\110\001\260\042\226\100\020\106\264\261\147\054"
             "\251\165\375\053",
     (PRUint32)20},
    {CKA_CERT_MD5_HASH,
     (void *)"\160\325\060\361\332\224\227\324\327\164\337\276\355\150\336\226",
     (PRUint32)16},
    {CKA_ISSUER,
     (void *)"\060\201\224\061\013\060\011\006\003\125\004\006\023\002\125\123"
             "\061\035\060\033\006\003\125\004\012\023\024\123\171\155\141\156"
             "\164\145\143\040\103\157\162\160\157\162\141\164\151\157\156\061"
             "\037\060\035\006\003\125\004\013\023\026\123\171\155\141\156\164"
             "\145\143\040\124\162\165\163\164\040\116\145\164\167\157\162\153"
             "\061\105\060\103\006\003\125\004\003\023\074\123\171\155\141\156"
             "\164\145\143\040\103\154\141\163\163\040\062\040\120\165\142\154"
             "\151\143\040\120\162\151\155\141\162\171\040\103\145\162\164\151"
             "\146\151\143\141\164\151\157\156\040\101\165\164\150\157\162\151"
             "\164\171\040\055\040\107\064",
     (PRUint32)151},
    {CKA_SERIAL_NUMBER,
     (void *)"\002\020\064\027\145\022\100\073\267\126\200\055\200\313\171\125"
             "\246\036",
     (PRUint32)18},
    {CKA_TRUST_SERVER_AUTH, (void *)&ckt_nss_must_verify_trust,
     (PRUint32)sizeof(CK_TRUST)},
    {CKA_TRUST_EMAIL_PROTECTION, (void *)&ckt_nss_trusted_delegator,
     (PRUint32)sizeof(CK_TRUST)},
    {CKA_TRUST_CODE_SIGNING, (void *)&ckt_nss_must_verify_trust,
     (PRUint32)sizeof(CK_TRUST)},
    {CKA_TRUST_STEP_UP_APPROVED, (void *)&ck_false,
     (PRUint32)sizeof(CK_BBOOL)}};

TEST_F(SoftokenTest, CreateObjectNonEmptyPassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, "password"));
  EXPECT_EQ(SECSuccess, PK11_Logout(slot.get()));
  ScopedPK11GenericObject obj(PK11_CreateGenericObject(
      slot.get(), attributes, PR_ARRAY_SIZE(attributes), true));
  EXPECT_EQ(nullptr, obj);
}

TEST_F(SoftokenTest, CreateObjectChangePassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
  EXPECT_EQ(SECSuccess, PK11_ChangePW(slot.get(), "", "password"));
  EXPECT_EQ(SECSuccess, PK11_Logout(slot.get()));
  ScopedPK11GenericObject obj(PK11_CreateGenericObject(
      slot.get(), attributes, PR_ARRAY_SIZE(attributes), true));
  EXPECT_EQ(nullptr, obj);
}

/* The size limit for a password is 500 characters as defined in pkcs11i.h */
TEST_F(SoftokenTest, CreateObjectChangeToBigPassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
  EXPECT_EQ(
      SECSuccess,
      PK11_ChangePW(slot.get(), "",
                    "rUIFIFr2bxKnbJbitsfkyqttpk6vCJzlYMNxcxXcaN37gSZKbLk763X7iR"
                    "yeVNWZHQ02lSF69HYjzTyPW3318ZD0DBFMMbALZ8ZPZP73CIo5uIQlaowV"
                    "IbP8eOhRYtGUqoLGlcIFNEYogV8Q3GN58VeBMs0KxrIOvPQ9s8SnYYkqvt"
                    "zzgntmAvCgvk64x6eQf0okHwegd5wi6m0WVJytEepWXkP9J629FSa5kNT8"
                    "FvL3jvslkiImzTNuTvl32fQDXXMSc8vVk5Q3mH7trMZM0VDdwHWYERjHbz"
                    "kGxFgp0VhediHx7p9kkz6H6ac4et9sW4UkTnN7xhYc1Zr17wRSk2heQtcX"
                    "oZJGwuzhiKm8A8wkuVxms6zO56P4JORIk8oaUW6lyNTLo2kWWnTA"));
  EXPECT_EQ(SECSuccess, PK11_Logout(slot.get()));
  ScopedPK11GenericObject obj(PK11_CreateGenericObject(
      slot.get(), attributes, PR_ARRAY_SIZE(attributes), true));
  EXPECT_EQ(nullptr, obj);
}

TEST_F(SoftokenTest, CreateObjectChangeToEmptyPassword) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, "password"));
  EXPECT_EQ(SECSuccess, PK11_ChangePW(slot.get(), "password", ""));
  // PK11_Logout returnes an error and SEC_ERROR_TOKEN_NOT_LOGGED_IN if the user
  // is not "logged in".
  EXPECT_EQ(SECFailure, PK11_Logout(slot.get()));
  EXPECT_EQ(SEC_ERROR_TOKEN_NOT_LOGGED_IN, PORT_GetError());
  ScopedPK11GenericObject obj(PK11_CreateGenericObject(
      slot.get(), attributes, PR_ARRAY_SIZE(attributes), true));
  // Because there's no password we can't logout and the operation should have
  // succeeded.
  EXPECT_NE(nullptr, obj);
}

class SoftokenNonAsciiTest : public SoftokenTest {
 protected:
  SoftokenNonAsciiTest() : SoftokenTest("SoftokenTest.\xF7-") {}
};

TEST_F(SoftokenNonAsciiTest, NonAsciiPathWorking) {
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
  EXPECT_EQ(SECSuccess, PK11_ResetToken(slot.get(), nullptr));
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, nullptr));
}

// This is just any X509 certificate. Its contents don't matter.
static unsigned char certDER[] = {
    0x30, 0x82, 0x01, 0xEF, 0x30, 0x82, 0x01, 0x94, 0xA0, 0x03, 0x02, 0x01,
    0x02, 0x02, 0x14, 0x49, 0xC4, 0xC4, 0x4A, 0xB6, 0x86, 0x07, 0xA3, 0x06,
    0xDC, 0x4D, 0xC8, 0xC3, 0xFE, 0xC7, 0x21, 0x3A, 0x2D, 0xE4, 0xDA, 0x30,
    0x0B, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B,
    0x30, 0x0F, 0x31, 0x0D, 0x30, 0x0B, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0C,
    0x04, 0x74, 0x65, 0x73, 0x74, 0x30, 0x22, 0x18, 0x0F, 0x32, 0x30, 0x31,
    0x35, 0x31, 0x31, 0x32, 0x38, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A,
    0x18, 0x0F, 0x32, 0x30, 0x31, 0x38, 0x30, 0x32, 0x30, 0x35, 0x30, 0x30,
    0x30, 0x30, 0x30, 0x30, 0x5A, 0x30, 0x0F, 0x31, 0x0D, 0x30, 0x0B, 0x06,
    0x03, 0x55, 0x04, 0x03, 0x0C, 0x04, 0x74, 0x65, 0x73, 0x74, 0x30, 0x82,
    0x01, 0x22, 0x30, 0x0D, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D,
    0x01, 0x01, 0x01, 0x05, 0x00, 0x03, 0x82, 0x01, 0x0F, 0x00, 0x30, 0x82,
    0x01, 0x0A, 0x02, 0x82, 0x01, 0x01, 0x00, 0xBA, 0x88, 0x51, 0xA8, 0x44,
    0x8E, 0x16, 0xD6, 0x41, 0xFD, 0x6E, 0xB6, 0x88, 0x06, 0x36, 0x10, 0x3D,
    0x3C, 0x13, 0xD9, 0xEA, 0xE4, 0x35, 0x4A, 0xB4, 0xEC, 0xF5, 0x68, 0x57,
    0x6C, 0x24, 0x7B, 0xC1, 0xC7, 0x25, 0xA8, 0xE0, 0xD8, 0x1F, 0xBD, 0xB1,
    0x9C, 0x06, 0x9B, 0x6E, 0x1A, 0x86, 0xF2, 0x6B, 0xE2, 0xAF, 0x5A, 0x75,
    0x6B, 0x6A, 0x64, 0x71, 0x08, 0x7A, 0xA5, 0x5A, 0xA7, 0x45, 0x87, 0xF7,
    0x1C, 0xD5, 0x24, 0x9C, 0x02, 0x7E, 0xCD, 0x43, 0xFC, 0x1E, 0x69, 0xD0,
    0x38, 0x20, 0x29, 0x93, 0xAB, 0x20, 0xC3, 0x49, 0xE4, 0xDB, 0xB9, 0x4C,
    0xC2, 0x6B, 0x6C, 0x0E, 0xED, 0x15, 0x82, 0x0F, 0xF1, 0x7E, 0xAD, 0x69,
    0x1A, 0xB1, 0xD3, 0x02, 0x3A, 0x8B, 0x2A, 0x41, 0xEE, 0xA7, 0x70, 0xE0,
    0x0F, 0x0D, 0x8D, 0xFD, 0x66, 0x0B, 0x2B, 0xB0, 0x24, 0x92, 0xA4, 0x7D,
    0xB9, 0x88, 0x61, 0x79, 0x90, 0xB1, 0x57, 0x90, 0x3D, 0xD2, 0x3B, 0xC5,
    0xE0, 0xB8, 0x48, 0x1F, 0xA8, 0x37, 0xD3, 0x88, 0x43, 0xEF, 0x27, 0x16,
    0xD8, 0x55, 0xB7, 0x66, 0x5A, 0xAA, 0x7E, 0x02, 0x90, 0x2F, 0x3A, 0x7B,
    0x10, 0x80, 0x06, 0x24, 0xCC, 0x1C, 0x6C, 0x97, 0xAD, 0x96, 0x61, 0x5B,
    0xB7, 0xE2, 0x96, 0x12, 0xC0, 0x75, 0x31, 0xA3, 0x0C, 0x91, 0xDD, 0xB4,
    0xCA, 0xF7, 0xFC, 0xAD, 0x1D, 0x25, 0xD3, 0x09, 0xEF, 0xB9, 0x17, 0x0E,
    0xA7, 0x68, 0xE1, 0xB3, 0x7B, 0x2F, 0x22, 0x6F, 0x69, 0xE3, 0xB4, 0x8A,
    0x95, 0x61, 0x1D, 0xEE, 0x26, 0xD6, 0x25, 0x9D, 0xAB, 0x91, 0x08, 0x4E,
    0x36, 0xCB, 0x1C, 0x24, 0x04, 0x2C, 0xBF, 0x16, 0x8B, 0x2F, 0xE5, 0xF1,
    0x8F, 0x99, 0x17, 0x31, 0xB8, 0xB3, 0xFE, 0x49, 0x23, 0xFA, 0x72, 0x51,
    0xC4, 0x31, 0xD5, 0x03, 0xAC, 0xDA, 0x18, 0x0A, 0x35, 0xED, 0x8D, 0x02,
    0x03, 0x01, 0x00, 0x01, 0x30, 0x0B, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86,
    0xF7, 0x0D, 0x01, 0x01, 0x0B, 0x03, 0x48, 0x00, 0x30, 0x45, 0x02, 0x20,
    0x5C, 0x75, 0x51, 0x9F, 0x13, 0x11, 0x50, 0xCD, 0x5D, 0x8A, 0xDE, 0x20,
    0xA3, 0xBC, 0x06, 0x30, 0x91, 0xFF, 0xB2, 0x73, 0x75, 0x5F, 0x31, 0x64,
    0xEC, 0xFD, 0xCB, 0x42, 0x80, 0x0A, 0x70, 0xE6, 0x02, 0x21, 0x00, 0x82,
    0x12, 0xF7, 0xE5, 0xEA, 0x40, 0x27, 0xFD, 0xF7, 0xC0, 0x0E, 0x25, 0xF3,
    0x3E, 0x34, 0x95, 0x80, 0xB9, 0xA3, 0x38, 0xE0, 0x56, 0x68, 0xDA, 0xE5,
    0xC1, 0xF5, 0x37, 0xC7, 0xB5, 0xCE, 0x0D};

struct PasswordPair {
  const char *mInitialPassword;
  const char *mSecondPassword;
};

class SoftokenPasswordChangeTest
    : public SoftokenTest,
      public ::testing::WithParamInterface<PasswordPair> {};

TEST_P(SoftokenPasswordChangeTest, KeepTrustAfterPasswordChange) {
  const PasswordPair &passwords = GetParam();
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  // Set a password.
  EXPECT_EQ(SECSuccess,
            PK11_InitPin(slot.get(), nullptr, passwords.mInitialPassword));
  SECItem certDERItem = {siBuffer, certDER, sizeof(certDER)};
  // Import a certificate.
  ScopedCERTCertificate cert(CERT_NewTempCertificate(
      CERT_GetDefaultCertDB(), &certDERItem, nullptr, true, true));
  EXPECT_TRUE(cert);
  SECStatus result =
      PK11_ImportCert(slot.get(), cert.get(), CK_INVALID_HANDLE, "test", false);
  EXPECT_EQ(SECSuccess, result);
  // Set a trust value.
  CERTCertTrust trust = {CERTDB_TRUSTED_CLIENT_CA | CERTDB_NS_TRUSTED_CA |
                             CERTDB_TRUSTED_CA | CERTDB_VALID_CA,
                         0, 0};
  result = CERT_ChangeCertTrust(nullptr, cert.get(), &trust);
  EXPECT_EQ(SECSuccess, result);
  // Release the certificate to ensure we get it from the DB rather than an
  // in-memory cache, below.
  cert = nullptr;
  // Change the password.
  result = PK11_ChangePW(slot.get(), passwords.mInitialPassword,
                         passwords.mSecondPassword);
  EXPECT_EQ(SECSuccess, result);
  // Look up the certificate again.
  ScopedCERTCertificate newCert(
      PK11_FindCertFromDERCertItem(slot.get(), &certDERItem, nullptr));
  EXPECT_TRUE(newCert.get());
  // The trust should be the same as before.
  CERTCertTrust newTrust = {0, 0, 0};
  result = CERT_GetCertTrust(newCert.get(), &newTrust);
  EXPECT_EQ(SECSuccess, result);
  EXPECT_EQ(trust.sslFlags, newTrust.sslFlags);
  EXPECT_EQ(trust.emailFlags, newTrust.emailFlags);
  EXPECT_EQ(trust.objectSigningFlags, newTrust.objectSigningFlags);
}

static const PasswordPair PASSWORD_CHANGE_TESTS[] = {
    {"password", ""},           // non-empty to empty password
    {"", "password"},           // empty to non-empty password
    {"password", "password2"},  // non-empty to non-empty password
};

INSTANTIATE_TEST_CASE_P(SoftokenPasswordChangeTests, SoftokenPasswordChangeTest,
                        ::testing::ValuesIn(PASSWORD_CHANGE_TESTS));

class SoftokenNoDBTest : public ::testing::Test {};

TEST_F(SoftokenNoDBTest, NeedUserInitNoDB) {
  ASSERT_EQ(SECSuccess, NSS_NoDB_Init("."));
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);
  EXPECT_EQ(PR_FALSE, PK11_NeedUserInit(slot.get()));

  // When shutting down in here we have to release the slot first.
  slot = nullptr;
  ASSERT_EQ(SECSuccess, NSS_Shutdown());
}

#ifndef NSS_FIPS_DISABLED

class SoftokenFipsTest : public SoftokenTest {
 protected:
  SoftokenFipsTest() : SoftokenTest("SoftokenFipsTest.d-") {}

  virtual void SetUp() {
    SoftokenTest::SetUp();

    // Turn on FIPS mode (code borrowed from FipsMode in modutil/pk11.c)
    char *internal_name;
    ASSERT_FALSE(PK11_IsFIPS());
    internal_name = PR_smprintf("%s", SECMOD_GetInternalModule()->commonName);
    ASSERT_EQ(SECSuccess, SECMOD_DeleteInternalModule(internal_name));
    PR_smprintf_free(internal_name);
    ASSERT_TRUE(PK11_IsFIPS());
  }
};

const std::vector<std::string> kFipsPasswordCases[] = {
    // FIPS level1 -> level1 -> level1
    {"", "", ""},
    // FIPS level1 -> level1 -> level2
    {"", "", "strong-_123"},
    // FIXME: this should work: FIPS level1 -> level2 -> level2
    // {"", "strong-_123", "strong-_456"},
    // FIPS level2 -> level2 -> level2
    {"strong-_123", "strong-_456", "strong-_123"}};

const std::vector<std::string> kFipsPasswordBadCases[] = {
    // FIPS level1 -> level2 -> level1
    {"", "strong-_123", ""},
    // FIPS level2 -> level1 -> level1
    {"strong-_123", ""},
    // FIPS level2 -> level2 -> level1
    {"strong-_123", "strong-_456", ""},
    // initialize with a weak password
    {"weak"},
    // FIPS level1 -> weak password
    {"", "weak"},
    // FIPS level2 -> weak password
    {"strong-_123", "weak"}};

class SoftokenFipsPasswordTest
    : public SoftokenFipsTest,
      public ::testing::WithParamInterface<std::vector<std::string>> {};

class SoftokenFipsBadPasswordTest
    : public SoftokenFipsTest,
      public ::testing::WithParamInterface<std::vector<std::string>> {};

TEST_P(SoftokenFipsPasswordTest, SetPassword) {
  const std::vector<std::string> &passwords = GetParam();
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);

  auto it = passwords.begin();
  auto prev_it = it;
  EXPECT_EQ(SECSuccess, PK11_InitPin(slot.get(), nullptr, (*it).c_str()));
  for (it++; it != passwords.end(); it++, prev_it++) {
    EXPECT_EQ(SECSuccess,
              PK11_ChangePW(slot.get(), (*prev_it).c_str(), (*it).c_str()));
  }
}

TEST_P(SoftokenFipsBadPasswordTest, SetBadPassword) {
  const std::vector<std::string> &passwords = GetParam();
  ScopedPK11SlotInfo slot(PK11_GetInternalKeySlot());
  ASSERT_TRUE(slot);

  auto it = passwords.begin();
  auto prev_it = it;
  SECStatus rv = PK11_InitPin(slot.get(), nullptr, (*it).c_str());
  if (it + 1 == passwords.end())
    EXPECT_EQ(SECFailure, rv);
  else
    EXPECT_EQ(SECSuccess, rv);
  for (it++; it != passwords.end(); it++, prev_it++) {
    rv = PK11_ChangePW(slot.get(), (*prev_it).c_str(), (*it).c_str());
    if (it + 1 == passwords.end())
      EXPECT_EQ(SECFailure, rv);
    else
      EXPECT_EQ(SECSuccess, rv);
  }
}

INSTANTIATE_TEST_CASE_P(FipsPasswordCases, SoftokenFipsPasswordTest,
                        ::testing::ValuesIn(kFipsPasswordCases));

INSTANTIATE_TEST_CASE_P(BadFipsPasswordCases, SoftokenFipsBadPasswordTest,
                        ::testing::ValuesIn(kFipsPasswordBadCases));

#endif

}  // namespace nss_test

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);

  return RUN_ALL_TESTS();
}
