// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.
use crate::mysql_demo::tests::test_util::log_init;
use crate::mysql_demo::tests::test_util::{
    directory_test_suite_key_history_verify, directory_test_suite_key_history_verify_v2,
};
use akd::{
    ecvrf::HardCodedAkdVRF, storage::StorageManager, ExampleLabel, ExperimentalConfiguration,
    WhatsAppV1Configuration,
};
use log::info;

type ExperimentalConfigType = ExperimentalConfiguration<ExampleLabel>;

type InMemoryDb = akd::storage::memory::AsyncInMemoryDatabase;

macro_rules! test_directory_operations {
    ($proof:ident, $verify:ident, $($tc:ident),*) => {
        $(paste::paste! {
        #[serial_test::serial]
        #[tokio::test]
        async fn [<test_directory_operations _ $verify _ $tc:lower>]() {
            log_init(log::Level::Info);

            info!("\n\n******** Starting In-Memory Directory Operations Integration Test ********\n\n");

            let db = InMemoryDb::new();

            let vrf = HardCodedAkdVRF {};
            let storage_manager = StorageManager::new_no_cache(db);
            [<directory_test_suite _ $verify>]::<$tc, _, HardCodedAkdVRF>(&storage_manager, 500, &vrf).await;

            info!("\n\n******** Finished In-Memory Directory Operations Integration Test ********\n\n");
        }
        })*
    }
}

test_directory_operations!(
    key_history,
    key_history_verify,
    WhatsAppV1Configuration,
    ExperimentalConfigType
);
test_directory_operations!(
    key_history_v2,
    key_history_verify_v2,
    WhatsAppV1Configuration,
    ExperimentalConfigType
);

macro_rules! test_directory_operations_with_caching {
    ($proof:ident, $verify:ident, $($tc:ident),*) => {
        $(paste::paste! {
        #[serial_test::serial]
        #[tokio::test]
        async fn [<test_directory_operations_with_caching _ $verify _ $tc:lower>]() {
            log_init(log::Level::Info);

            info!("\n\n******** Starting In-Memory Directory Operations (w/caching) Integration Test ********\n\n");

            let db = InMemoryDb::new();

            let vrf = HardCodedAkdVRF {};
            let storage_manager = StorageManager::new(db, None, None, None);
            [<directory_test_suite _ $verify>]::<$tc, _, HardCodedAkdVRF>(&storage_manager, 500, &vrf).await;

            info!("\n\n******** Finished In-Memory Directory Operations (w/caching) Integration Test ********\n\n");
        }
        })*
    }
}

test_directory_operations_with_caching!(
    key_history,
    key_history_verify,
    WhatsAppV1Configuration,
    ExperimentalConfigType
);
test_directory_operations_with_caching!(
    key_history_v2,
    key_history_verify_v2,
    WhatsAppV1Configuration,
    ExperimentalConfigType
);
