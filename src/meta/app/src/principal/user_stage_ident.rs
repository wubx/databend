// Copyright 2021 Datafuse Labs
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

use crate::tenant::Tenant;

/// Define the meta-service key for a stage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StageIdent {
    pub tenant: Tenant,
    pub name: String,
}

impl StageIdent {
    pub fn new(tenant: Tenant, name: impl ToString) -> Self {
        Self {
            tenant,
            name: name.to_string(),
        }
    }
}

mod kvapi_key_impl {
    use databend_common_meta_kvapi::kvapi;
    use databend_common_meta_kvapi::kvapi::KeyError;

    use crate::principal::user_stage_ident::StageIdent;
    use crate::principal::StageInfo;
    use crate::tenant::Tenant;
    use crate::KeyWithTenant;

    impl kvapi::Key for StageIdent {
        const PREFIX: &'static str = "__fd_stages";
        type ValueType = StageInfo;

        fn parent(&self) -> Option<String> {
            Some(self.tenant.to_string_key())
        }

        fn encode_key(&self, b: kvapi::KeyBuilder) -> kvapi::KeyBuilder {
            b.push_str(self.tenant_name()).push_str(&self.name)
        }

        fn decode_key(p: &mut kvapi::KeyParser) -> Result<Self, KeyError> {
            let tenant = p.next_nonempty()?;
            let name = p.next_str()?;

            Ok(StageIdent::new(Tenant::new_nonempty(tenant), name))
        }
    }

    impl KeyWithTenant for StageIdent {
        fn tenant(&self) -> &Tenant {
            &self.tenant
        }
    }

    impl kvapi::Value for StageInfo {
        fn dependency_keys(&self) -> impl IntoIterator<Item = String> {
            []
        }
    }
}

#[cfg(test)]
mod tests {
    use databend_common_meta_kvapi::kvapi::Key;

    use crate::principal::user_stage_ident::StageIdent;
    use crate::tenant::Tenant;

    #[test]
    fn test_kvapi_key_for_stage_ident() {
        let tenant = Tenant::new("test");
        let stage = StageIdent::new(tenant, "stage");

        let key = stage.to_string_key();
        assert_eq!(key, "__fd_stages/test/stage");
        assert_eq!(stage, StageIdent::from_str_key(&key).unwrap());
    }
}
