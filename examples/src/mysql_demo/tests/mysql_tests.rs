// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is dual-licensed under either the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree or the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree. You may select, at your option, one of the above-listed licenses.

use crate::mysql_demo::mysql::AsyncMySqlDatabase;
use crate::mysql_demo::tests::test_util::{
    log_init, test_lookups as test_lookups_util,
};
use crate::test_config_serial;
use akd::storage::StorageManager;
use akd::{
    ecvrf::HardCodedAkdVRF, Configuration, ExampleLabel, ExperimentalConfiguration,
    WhatsAppV1Configuration,
};
use log::{error, info, warn};
type ExperimentalConfigType = ExperimentalConfiguration<ExampleLabel>;

use crate::mysql_demo::tests::test_util::{directory_test_suite_key_history_verify, directory_test_suite_key_history_verify_v2};

macro_rules! test_directory_operations {
    ($proof:ident, $verify:ident, $($tc:ident),*) => {
        $(paste::paste! {
        #[serial_test::serial]
        #[tokio::test]
        async fn [<test_directory_operations _ $verify _ $tc:lower>]() {
            log_init(log::Level::Info);

            info!("\n\n******** Starting MySQL Directory Operations Integration Test ********\n\n");

            if AsyncMySqlDatabase::test_guard() {
                // create the "test" database
                if let Err(error) = AsyncMySqlDatabase::create_test_db(
                    "localhost",
                    Option::from("root"),
                    Option::from("example"),
                    Option::from(8001),
                )
                .await
                {
                    panic!("Error creating test database: {}", error);
                }

                // connect to the newly created test db
                let mysql_db = AsyncMySqlDatabase::new(
                    "localhost",
                    "test_db",
                    Option::from("root"),
                    Option::from("example"),
                    Option::from(8001),
                    200,
                )
                .await
                .expect("Failed to create async mysql db");

                // delete all data from the db
                if let Err(error) = mysql_db.delete_data().await {
                    error!("Error cleaning mysql prior to test suite: {}", error);
                }

                let vrf = HardCodedAkdVRF {};
                let storage_manager = StorageManager::new_no_cache(mysql_db.clone());
                [<directory_test_suite _ $verify>]::<$tc, _, HardCodedAkdVRF>(&storage_manager, 50, &vrf).await;

                storage_manager.log_metrics(log::Level::Trace).await;

                // clean the test infra
                if let Err(mysql_async::Error::Server(error)) = storage_manager.get_db().drop_tables().await
                {
                    error!(
                        "ERROR: Failed to clean MySQL test database with error {}",
                        error
                    );
                }
                } else {
                    warn!("WARN: Skipping MySQL test due to test guard noting that the docker container appears to not be running.");
                }

                info!("\n\n******** Completed MySQL Directory Operations Integration Test ********\n\n");
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
            info!("\n\n******** Starting MySQL Directory Operations (w/caching) Integration Test ********\n\n");

            if AsyncMySqlDatabase::test_guard() {
                // create the "test" database
                if let Err(error) = AsyncMySqlDatabase::create_test_db(
                    "localhost",
                    Option::from("root"),
                    Option::from("example"),
                    Option::from(8001),
                )
                .await
                {
                    panic!("Error creating test database: {}", error);
                }

                // connect to the newly created test db
                let mysql_db = AsyncMySqlDatabase::new(
                    "localhost",
                    "test_db",
                    Option::from("root"),
                    Option::from("example"),
                    Option::from(8001),
                    200,
                )
                .await
                .expect("Failed to create async mysql db");

                // delete all data from the db
                if let Err(error) = mysql_db.delete_data().await {
                    error!("Error cleaning mysql prior to test suite: {}", error);
                }

                let vrf = HardCodedAkdVRF {};
                let storage_manager = StorageManager::new(mysql_db.clone(), None, None, None);
                [<directory_test_suite _ $verify>]::<$tc, _, HardCodedAkdVRF>(&storage_manager, 50, &vrf).await;

                storage_manager.log_metrics(log::Level::Trace).await;

                // clean the test infra
                if let Err(mysql_async::Error::Server(error)) = storage_manager.get_db().drop_tables().await
                {
                    error!(
                        "ERROR: Failed to clean MySQL test database with error {}",
                        error
                    );
                }
            } else {
                warn!("WARN: Skipping MySQL test due to test guard noting that the docker container appears to not be running.");
            }

            info!("\n\n******** Completed MySQL Directory Operations (w/caching) Integration Test ********\n\n");
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

test_config_serial!(test_lookups);
async fn test_lookups<TC: Configuration>() {
    log_init(log::Level::Info);

    info!("\n\n******** Starting MySQL Lookup Tests ********\n\n");

    if AsyncMySqlDatabase::test_guard() {
        // create the "test" database
        if let Err(error) = AsyncMySqlDatabase::create_test_db(
            "localhost",
            Option::from("root"),
            Option::from("example"),
            Option::from(8001),
        )
        .await
        {
            panic!("Error creating test database: {}", error);
        }

        // connect to the newly created test db
        let mysql_db = AsyncMySqlDatabase::new(
            "localhost",
            "test_db",
            Option::from("root"),
            Option::from("example"),
            Option::from(8001),
            200,
        )
        .await
        .expect("Failed to create async mysql db");

        // delete all data from the db
        if let Err(error) = mysql_db.delete_data().await {
            error!("Error cleaning mysql prior to test suite: {}", error);
        }

        let vrf = HardCodedAkdVRF {};
        let storage_manager = StorageManager::new(mysql_db, None, None, None);

        test_lookups_util::<TC, _, HardCodedAkdVRF>(&storage_manager, &vrf, 50, 5, 100).await;

        // clean the test infra
        if let Err(mysql_async::Error::Server(error)) = storage_manager.get_db().drop_tables().await
        {
            error!(
                "ERROR: Failed to clean MySQL test database with error {}",
                error
            );
        }
    } else {
        warn!("WARN: Skipping MySQL test due to test guard noting that the docker container appears to not be running.");
    }

    info!("\n\n******** Completed MySQL Lookup Tests ********\n\n");
}
