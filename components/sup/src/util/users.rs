// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// TODO
extern crate users;
use users::*;

use error::{Result, Error};
use package::Package;

/// This function checks to see if a custom SVC_USER and SVC_GROUP has
/// been specified as part of the package metadata. If not, return None
fn check_pkg_user_and_group(pkg: &Package) -> Result<Option<(String, String)>> {
    let svc_user = try!(pkg.pkg_install.svc_user());
    let svc_group = try!(pkg.pkg_install.svc_group());
    match (svc_user, svc_group) {
        (Some(user), Some(group)) => {
            // a package has a SVC_USER and SVC_GROUP defined,
            // these MUST exist in order to continue
            debug!("SVC_USER = {}", &user);
            debug!("SVC_GROUP = {}", &group);
            debug!("Checking to see if user and group exist");
            if let None = users::get_user_by_name(&user) {
                panic!("Package requires user {} to exist, but it doesn't", user);
                // TODO: return an Error
            }
            if let None = users::get_group_by_name(&group) {
                panic!("Package requires group {} to exist, but it doesn't", group);
                // TODO: return an Error
            }
            Ok(Some((user, group)))
        }
        _ => {
            debug!("User/group not specified in package, running with default");
            Ok(None)
        }
    }
}

/// checks to see if hab/hab exists, if not, fall back to
/// current user/group. If that fails, then return an error.
fn get_default_user_and_group() -> Result<(String, String)> {
    // TODO: constants
    let user = users::get_user_by_name("hab");
    let group = users::get_group_by_name("hab");
    match (user, group) {
        (Some(user), Some(group)) => return Ok((user.name().to_string(), group.name().to_string())),
        _ => {
            println!("hab/hab does NOT exist");
            let user = users::get_current_username();
            let group = users::get_current_groupname();
            match (user, group) {
                (Some(user), Some(group)) => {
                    println!("Running as {}/{}", user, group);
                    return Ok((user, group));
                }
                _ => {
                    println!("Can't determine current user/group");
                    // TODO: error handling
                    return Ok(("nobody".to_string(), "nobody".to_string()));
                }
            }
        }
    }
}

/// check and see if a user/group is specified in package metadata.
/// if not, we'll try and use hab/hab.
/// If hab/hab doesn't exist, try to use (current username, current group).
/// If that doesn't work, then give up.
#[cfg(any(target_os="linux", target_os="macos"))]
pub fn get_user_and_group(pkg: &Package) -> Result<(String, String)> {
    if let Some((user, group)) = try!(check_pkg_user_and_group(&pkg)) {
        Ok((user, group))
    } else {
        let defaults = try!(get_default_user_and_group());
        Ok(defaults)
    }
}

#[cfg(any(target_os="linux", target_os="macos"))]
pub fn user_name_to_uid(user : &str) -> Option<u32> {
    users::get_user_by_name(user).map(|u| u.uid())
}

#[cfg(any(target_os="linux", target_os="macos"))]
pub fn group_name_to_gid(group: &str) -> Option<u32> {
    users::get_group_by_name(group).map(|g| g.gid())
}

#[cfg(target_os = "windows")]
pub fn get_user_and_group(pkg: &Package) -> Result<(String, String)> {
    unimplemented!();
}



