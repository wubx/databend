// Copyright 2021 Datafuse Labs.
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

mod logic;
mod logic_and;
mod logic_not;
mod logic_or;

pub use logic::LogicFunction;
pub use logic_and::LogicAndFunction;
pub use logic_not::LogicNotFunction;
pub use logic_or::LogicOrFunction;

mod logic2;
mod logic2_and;
mod logic2_or;
mod logic2_xor;
mod logic2_not;

pub use logic2::LogicFunction2;
pub use logic2_and::LogicAndFunction2;
pub use logic2_or::LogicOrFunction2;
pub use logic2_not::LogicNotFunction2;
pub use logic2_xor::LogicXorFunction2;
