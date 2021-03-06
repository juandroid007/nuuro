// Copyright 2020-2020 Juan Villacorta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u16)]
pub enum {0} {{
{1}}}

impl IdU16 for {0} {{
    fn id_u16(self) -> u16 {{ self as u16 }}
    fn count() -> u16 {{ {2} }}
    fn from_u16(id: u16) -> Option<Self> {{
        if id < Self::count() {{ Some(unsafe {{ mem::transmute(id) }}) }} else {{ None }}
    }}
}}
