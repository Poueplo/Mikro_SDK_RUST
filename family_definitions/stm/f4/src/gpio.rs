/****************************************************************************
**
** Copyright (C) ${COPYRIGHT_YEAR} MikroElektronika d.o.o.
** Contact: https://www.mikroe.com/contact
**
** This file is part of the mikroSDK package
**
** Commercial License Usage
**
** Licensees holding valid commercial NECTO compilers AI licenses may use this
** file in accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The MikroElektronika Company.
** For licensing terms and conditions see
** https://www.mikroe.com/legal/software-license-agreement.
** For further information use the contact form at
** https://www.mikroe.com/contact.
**
**
** GNU Lesser General Public License Usage
**
** Alternatively, this file may be used for
** non-commercial projects under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** The above copyright notice and this permission notice shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
** OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
** DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
** OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
** OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**
****************************************************************************/

pub use hal_ll_target_names::*;

#[cfg(feature = "gpioa")]
pub const GPIOA_BASE_ADDR: u32 = 0x4002_0000;
#[cfg(feature = "gpiob")]
pub const GPIOB_BASE_ADDR: u32 = 0x4002_0400;
#[cfg(feature = "gpioc")]
pub const GPIOC_BASE_ADDR: u32 = 0x4002_0800;
#[cfg(feature = "gpiod")]
pub const GPIOD_BASE_ADDR: u32 = 0x4002_0C00;
#[cfg(feature = "gpioe")]
pub const GPIOE_BASE_ADDR: u32 = 0x4002_1000;
#[cfg(feature = "gpiof")]
pub const GPIOF_BASE_ADDR: u32 = 0x4002_1400;
#[cfg(feature = "gpiog")]
pub const GPIOG_BASE_ADDR: u32 = 0x4002_1800;
#[cfg(feature = "gpioh")]
pub const GPIOH_BASE_ADDR: u32 = 0x4002_1C00;
#[cfg(feature = "gpioi")]
pub const GPIOI_BASE_ADDR: u32 = 0x4002_2000;
#[cfg(feature = "gpioj")]
pub const GPIOJ_BASE_ADDR: u32 = 0x4002_2400;
#[cfg(feature = "gpiok")]
pub const GPIOK_BASE_ADDR: u32 = 0x4002_2800;

#[cfg(feature = "gpioa")]
pub const GPIO_PORT_A: hal_ll_port_name_t = 0x00;
#[cfg(feature = "gpiob")]
pub const GPIO_PORT_B: hal_ll_port_name_t = 0x01;
#[cfg(feature = "gpioc")]
pub const GPIO_PORT_C: hal_ll_port_name_t = 0x02;
#[cfg(feature = "gpiod")]
pub const GPIO_PORT_D: hal_ll_port_name_t = 0x03;
#[cfg(feature = "gpioe")]
pub const GPIO_PORT_E: hal_ll_port_name_t = 0x04;
#[cfg(feature = "gpiof")]
pub const GPIO_PORT_F: hal_ll_port_name_t = 0x05;
#[cfg(feature = "gpiog")]
pub const GPIO_PORT_G: hal_ll_port_name_t = 0x06;
#[cfg(feature = "gpioh")]
pub const GPIO_PORT_H: hal_ll_port_name_t = 0x07;
#[cfg(feature = "gpioi")]
pub const GPIO_PORT_I: hal_ll_port_name_t = 0x08;
#[cfg(feature = "gpioj")]
pub const GPIO_PORT_J: hal_ll_port_name_t = 0x09;
#[cfg(feature = "gpiok")]
pub const GPIO_PORT_K: hal_ll_port_name_t = 0x0A;

pub const PORT_SIZE: u8 = 16;

#[cfg(feature = "gpioa_0")]
pub const GPIO_A0: hal_ll_pin_name_t = 0x00;
#[cfg(feature = "gpioa_1")]
pub const GPIO_A1: hal_ll_pin_name_t = 0x01;
#[cfg(feature = "gpioa_2")]
pub const GPIO_A2: hal_ll_pin_name_t = 0x02;
#[cfg(feature = "gpioa_3")]
pub const GPIO_A3: hal_ll_pin_name_t = 0x03;
#[cfg(feature = "gpioa_4")]
pub const GPIO_A4: hal_ll_pin_name_t = 0x04;
#[cfg(feature = "gpioa_5")]
pub const GPIO_A5: hal_ll_pin_name_t = 0x05;
#[cfg(feature = "gpioa_6")]
pub const GPIO_A6: hal_ll_pin_name_t = 0x06;
#[cfg(feature = "gpioa_7")]
pub const GPIO_A7: hal_ll_pin_name_t = 0x07;
#[cfg(feature = "gpioa_8")]
pub const GPIO_A8: hal_ll_pin_name_t = 0x08;
#[cfg(feature = "gpioa_9")]
pub const GPIO_A9: hal_ll_pin_name_t = 0x09;
#[cfg(feature = "gpioa_10")]
pub const GPIO_A10: hal_ll_pin_name_t = 0x0A;
#[cfg(feature = "gpioa_11")]
pub const GPIO_A11: hal_ll_pin_name_t = 0x0B;
#[cfg(feature = "gpioa_12")]
pub const GPIO_A12: hal_ll_pin_name_t = 0x0C;
#[cfg(feature = "gpioa_13")]
pub const GPIO_A13: hal_ll_pin_name_t = 0x0D;
#[cfg(feature = "gpioa_14")]
pub const GPIO_A14: hal_ll_pin_name_t = 0x0E;
#[cfg(feature = "gpioa_15")]
pub const GPIO_A15: hal_ll_pin_name_t = 0x0F;

#[cfg(feature = "gpiob_0")]
pub const GPIO_B0: hal_ll_pin_name_t = 0x10;
#[cfg(feature = "gpiob_1")]
pub const GPIO_B1: hal_ll_pin_name_t = 0x11;
#[cfg(feature = "gpiob_2")]
pub const GPIO_B2: hal_ll_pin_name_t = 0x12;
#[cfg(feature = "gpiob_3")]
pub const GPIO_B3: hal_ll_pin_name_t = 0x13;
#[cfg(feature = "gpiob_4")]
pub const GPIO_B4: hal_ll_pin_name_t = 0x14;
#[cfg(feature = "gpiob_5")]
pub const GPIO_B5: hal_ll_pin_name_t = 0x15;
#[cfg(feature = "gpiob_6")]
pub const GPIO_B6: hal_ll_pin_name_t = 0x16;
#[cfg(feature = "gpiob_7")]
pub const GPIO_B7: hal_ll_pin_name_t = 0x17;
#[cfg(feature = "gpiob_8")]
pub const GPIO_B8: hal_ll_pin_name_t = 0x18;
#[cfg(feature = "gpiob_9")]
pub const GPIO_B9: hal_ll_pin_name_t = 0x19;
#[cfg(feature = "gpiob_10")]
pub const GPIO_B10: hal_ll_pin_name_t = 0x1A;
#[cfg(feature = "gpiob_11")]
pub const GPIO_B11: hal_ll_pin_name_t = 0x1B;
#[cfg(feature = "gpiob_12")]
pub const GPIO_B12: hal_ll_pin_name_t = 0x1C;
#[cfg(feature = "gpiob_13")]
pub const GPIO_B13: hal_ll_pin_name_t = 0x1D;
#[cfg(feature = "gpiob_14")]
pub const GPIO_B14: hal_ll_pin_name_t = 0x1E;
#[cfg(feature = "gpiob_15")]
pub const GPIO_B15: hal_ll_pin_name_t = 0x1F;

#[cfg(feature = "gpioc_0")]
pub const GPIO_C0: hal_ll_pin_name_t = 0x20;
#[cfg(feature = "gpioc_1")]
pub const GPIO_C1: hal_ll_pin_name_t = 0x21;
#[cfg(feature = "gpioc_2")]
pub const GPIO_C2: hal_ll_pin_name_t = 0x22;
#[cfg(feature = "gpioc_3")]
pub const GPIO_C3: hal_ll_pin_name_t = 0x23;
#[cfg(feature = "gpioc_4")]
pub const GPIO_C4: hal_ll_pin_name_t = 0x24;
#[cfg(feature = "gpioc_5")]
pub const GPIO_C5: hal_ll_pin_name_t = 0x25;
#[cfg(feature = "gpioc_6")]
pub const GPIO_C6: hal_ll_pin_name_t = 0x26;
#[cfg(feature = "gpioc_7")]
pub const GPIO_C7: hal_ll_pin_name_t = 0x27;
#[cfg(feature = "gpioc_8")]
pub const GPIO_C8: hal_ll_pin_name_t = 0x28;
#[cfg(feature = "gpioc_9")]
pub const GPIO_C9: hal_ll_pin_name_t = 0x29;
#[cfg(feature = "gpioc_10")]
pub const GPIO_C10: hal_ll_pin_name_t = 0x2A;
#[cfg(feature = "gpioc_11")]
pub const GPIO_C11: hal_ll_pin_name_t = 0x2B;
#[cfg(feature = "gpioc_12")]
pub const GPIO_C12: hal_ll_pin_name_t = 0x2C;
#[cfg(feature = "gpioc_13")]
pub const GPIO_C13: hal_ll_pin_name_t = 0x2D;
#[cfg(feature = "gpioc_14")]
pub const GPIO_C14: hal_ll_pin_name_t = 0x2E;
#[cfg(feature = "gpioc_15")]
pub const GPIO_C15: hal_ll_pin_name_t = 0x2F;

#[cfg(feature = "gpiod_0")]
pub const GPIO_D0: hal_ll_pin_name_t = 0x30;
#[cfg(feature = "gpiod_1")]
pub const GPIO_D1: hal_ll_pin_name_t = 0x31;
#[cfg(feature = "gpiod_2")]
pub const GPIO_D2: hal_ll_pin_name_t = 0x32;
#[cfg(feature = "gpiod_3")]
pub const GPIO_D3: hal_ll_pin_name_t = 0x33;
#[cfg(feature = "gpiod_4")]
pub const GPIO_D4: hal_ll_pin_name_t = 0x34;
#[cfg(feature = "gpiod_5")]
pub const GPIO_D5: hal_ll_pin_name_t = 0x35;
#[cfg(feature = "gpiod_6")]
pub const GPIO_D6: hal_ll_pin_name_t = 0x36;
#[cfg(feature = "gpiod_7")]
pub const GPIO_D7: hal_ll_pin_name_t = 0x37;
#[cfg(feature = "gpiod_8")]
pub const GPIO_D8: hal_ll_pin_name_t = 0x38;
#[cfg(feature = "gpiod_9")]
pub const GPIO_D9: hal_ll_pin_name_t = 0x39;
#[cfg(feature = "gpiod_10")]
pub const GPIO_D10: hal_ll_pin_name_t = 0x3A;
#[cfg(feature = "gpiod_11")]
pub const GPIO_D11: hal_ll_pin_name_t = 0x3B;
#[cfg(feature = "gpiod_12")]
pub const GPIO_D12: hal_ll_pin_name_t = 0x3C;
#[cfg(feature = "gpiod_13")]
pub const GPIO_D13: hal_ll_pin_name_t = 0x3D;
#[cfg(feature = "gpiod_14")]
pub const GPIO_D14: hal_ll_pin_name_t = 0x3E;
#[cfg(feature = "gpiod_15")]
pub const GPIO_D15: hal_ll_pin_name_t = 0x3F;

#[cfg(feature = "gpioe_0")]
pub const GPIO_E0: hal_ll_pin_name_t = 0x40;
#[cfg(feature = "gpioe_1")]
pub const GPIO_E1: hal_ll_pin_name_t = 0x41;
#[cfg(feature = "gpioe_2")]
pub const GPIO_E2: hal_ll_pin_name_t = 0x42;
#[cfg(feature = "gpioe_3")]
pub const GPIO_E3: hal_ll_pin_name_t = 0x43;
#[cfg(feature = "gpioe_4")]
pub const GPIO_E4: hal_ll_pin_name_t = 0x44;
#[cfg(feature = "gpioe_5")]
pub const GPIO_E5: hal_ll_pin_name_t = 0x45;
#[cfg(feature = "gpioe_6")]
pub const GPIO_E6: hal_ll_pin_name_t = 0x46;
#[cfg(feature = "gpioe_7")]
pub const GPIO_E7: hal_ll_pin_name_t = 0x47;
#[cfg(feature = "gpioe_8")]
pub const GPIO_E8: hal_ll_pin_name_t = 0x48;
#[cfg(feature = "gpioe_9")]
pub const GPIO_E9: hal_ll_pin_name_t = 0x49;
#[cfg(feature = "gpioe_10")]
pub const GPIO_E10: hal_ll_pin_name_t = 0x4A;
#[cfg(feature = "gpioe_11")]
pub const GPIO_E11: hal_ll_pin_name_t = 0x4B;
#[cfg(feature = "gpioe_12")]
pub const GPIO_E12: hal_ll_pin_name_t = 0x4C;
#[cfg(feature = "gpioe_13")]
pub const GPIO_E13: hal_ll_pin_name_t = 0x4D;
#[cfg(feature = "gpioe_14")]
pub const GPIO_E14: hal_ll_pin_name_t = 0x4E;
#[cfg(feature = "gpioe_15")]
pub const GPIO_E15: hal_ll_pin_name_t = 0x4F;

#[cfg(feature = "gpiof_0")]
pub const GPIO_F0: hal_ll_pin_name_t = 0x50;
#[cfg(feature = "gpiof_1")]
pub const GPIO_F1: hal_ll_pin_name_t = 0x51;
#[cfg(feature = "gpiof_2")]
pub const GPIO_F2: hal_ll_pin_name_t = 0x52;
#[cfg(feature = "gpiof_3")]
pub const GPIO_F3: hal_ll_pin_name_t = 0x53;
#[cfg(feature = "gpiof_4")]
pub const GPIO_F4: hal_ll_pin_name_t = 0x54;
#[cfg(feature = "gpiof_5")]
pub const GPIO_F5: hal_ll_pin_name_t = 0x55;
#[cfg(feature = "gpiof_6")]
pub const GPIO_F6: hal_ll_pin_name_t = 0x56;
#[cfg(feature = "gpiof_7")]
pub const GPIO_F7: hal_ll_pin_name_t = 0x57;
#[cfg(feature = "gpiof_8")]
pub const GPIO_F8: hal_ll_pin_name_t = 0x58;
#[cfg(feature = "gpiof_9")]
pub const GPIO_F9: hal_ll_pin_name_t = 0x59;
#[cfg(feature = "gpiof_10")]
pub const GPIO_F10: hal_ll_pin_name_t = 0x5A;
#[cfg(feature = "gpiof_11")]
pub const GPIO_F11: hal_ll_pin_name_t = 0x5B;
#[cfg(feature = "gpiof_12")]
pub const GPIO_F12: hal_ll_pin_name_t = 0x5C;
#[cfg(feature = "gpiof_13")]
pub const GPIO_F13: hal_ll_pin_name_t = 0x5D;
#[cfg(feature = "gpiof_14")]
pub const GPIO_F14: hal_ll_pin_name_t = 0x5E;
#[cfg(feature = "gpiof_15")]
pub const GPIO_F15: hal_ll_pin_name_t = 0x5F;

#[cfg(feature = "gpiog_0")]
pub const GPIO_G0: hal_ll_pin_name_t = 0x60;
#[cfg(feature = "gpiog_1")]
pub const GPIO_G1: hal_ll_pin_name_t = 0x61;
#[cfg(feature = "gpiog_2")]
pub const GPIO_G2: hal_ll_pin_name_t = 0x62;
#[cfg(feature = "gpiog_3")]
pub const GPIO_G3: hal_ll_pin_name_t = 0x63;
#[cfg(feature = "gpiog_4")]
pub const GPIO_G4: hal_ll_pin_name_t = 0x64;
#[cfg(feature = "gpiog_5")]
pub const GPIO_G5: hal_ll_pin_name_t = 0x65;
#[cfg(feature = "gpiog_6")]
pub const GPIO_G6: hal_ll_pin_name_t = 0x66;
#[cfg(feature = "gpiog_7")]
pub const GPIO_G7: hal_ll_pin_name_t = 0x67;
#[cfg(feature = "gpiog_8")]
pub const GPIO_G8: hal_ll_pin_name_t = 0x68;
#[cfg(feature = "gpiog_9")]
pub const GPIO_G9: hal_ll_pin_name_t = 0x69;
#[cfg(feature = "gpiog_10")]
pub const GPIO_G10: hal_ll_pin_name_t = 0x6A;
#[cfg(feature = "gpiog_11")]
pub const GPIO_G11: hal_ll_pin_name_t = 0x6B;
#[cfg(feature = "gpiog_12")]
pub const GPIO_G12: hal_ll_pin_name_t = 0x6C;
#[cfg(feature = "gpiog_13")]
pub const GPIO_G13: hal_ll_pin_name_t = 0x6D;
#[cfg(feature = "gpiog_14")]
pub const GPIO_G14: hal_ll_pin_name_t = 0x6E;
#[cfg(feature = "gpiog_15")]
pub const GPIO_G15: hal_ll_pin_name_t = 0x6F;

#[cfg(feature = "gpioh_0")]
pub const GPIO_H0: hal_ll_pin_name_t = 0x70;
#[cfg(feature = "gpioh_1")]
pub const GPIO_H1: hal_ll_pin_name_t = 0x71;
#[cfg(feature = "gpioh_2")]
pub const GPIO_H2: hal_ll_pin_name_t = 0x72;
#[cfg(feature = "gpioh_3")]
pub const GPIO_H3: hal_ll_pin_name_t = 0x73;
#[cfg(feature = "gpioh_4")]
pub const GPIO_H4: hal_ll_pin_name_t = 0x74;
#[cfg(feature = "gpioh_5")]
pub const GPIO_H5: hal_ll_pin_name_t = 0x75;
#[cfg(feature = "gpioh_6")]
pub const GPIO_H6: hal_ll_pin_name_t = 0x76;
#[cfg(feature = "gpioh_7")]
pub const GPIO_H7: hal_ll_pin_name_t = 0x77;
#[cfg(feature = "gpioh_8")]
pub const GPIO_H8: hal_ll_pin_name_t = 0x78;
#[cfg(feature = "gpioh_9")]
pub const GPIO_H9: hal_ll_pin_name_t = 0x79;
#[cfg(feature = "gpioh_10")]
pub const GPIO_H10: hal_ll_pin_name_t = 0x7A;
#[cfg(feature = "gpioh_11")]
pub const GPIO_H11: hal_ll_pin_name_t = 0x7B;
#[cfg(feature = "gpioh_12")]
pub const GPIO_H12: hal_ll_pin_name_t = 0x7C;
#[cfg(feature = "gpioh_13")]
pub const GPIO_H13: hal_ll_pin_name_t = 0x7D;
#[cfg(feature = "gpioh_14")]
pub const GPIO_H14: hal_ll_pin_name_t = 0x7E;
#[cfg(feature = "gpioh_15")]
pub const GPIO_H15: hal_ll_pin_name_t = 0x7F;

#[cfg(feature = "gpioi_0")]
pub const GPIO_I0: hal_ll_pin_name_t = 0x80;
#[cfg(feature = "gpioi_1")]
pub const GPIO_I1: hal_ll_pin_name_t = 0x81;
#[cfg(feature = "gpioi_2")]
pub const GPIO_I2: hal_ll_pin_name_t = 0x82;
#[cfg(feature = "gpioi_3")]
pub const GPIO_I3: hal_ll_pin_name_t = 0x83;
#[cfg(feature = "gpioi_4")]
pub const GPIO_I4: hal_ll_pin_name_t = 0x84;
#[cfg(feature = "gpioi_5")]
pub const GPIO_I5: hal_ll_pin_name_t = 0x85;
#[cfg(feature = "gpioi_6")]
pub const GPIO_I6: hal_ll_pin_name_t = 0x86;
#[cfg(feature = "gpioi_7")]
pub const GPIO_I7: hal_ll_pin_name_t = 0x87;
#[cfg(feature = "gpioi_8")]
pub const GPIO_I8: hal_ll_pin_name_t = 0x88;
#[cfg(feature = "gpioi_9")]
pub const GPIO_I9: hal_ll_pin_name_t = 0x89;
#[cfg(feature = "gpioi_10")]
pub const GPIO_I10: hal_ll_pin_name_t = 0x8A;
#[cfg(feature = "gpioi_11")]
pub const GPIO_I11: hal_ll_pin_name_t = 0x8B;
#[cfg(feature = "gpioi_12")]
pub const GPIO_I12: hal_ll_pin_name_t = 0x8C;
#[cfg(feature = "gpioi_13")]
pub const GPIO_I13: hal_ll_pin_name_t = 0x8D;
#[cfg(feature = "gpioi_14")]
pub const GPIO_I14: hal_ll_pin_name_t = 0x8E;
#[cfg(feature = "gpioi_15")]
pub const GPIO_I15: hal_ll_pin_name_t = 0x8F;

#[cfg(feature = "gpioj_0")]
pub const GPIO_J0: hal_ll_pin_name_t = 0x90;
#[cfg(feature = "gpioj_1")]
pub const GPIO_J1: hal_ll_pin_name_t = 0x91;
#[cfg(feature = "gpioj_2")]
pub const GPIO_J2: hal_ll_pin_name_t = 0x92;
#[cfg(feature = "gpioj_3")]
pub const GPIO_J3: hal_ll_pin_name_t = 0x93;
#[cfg(feature = "gpioj_4")]
pub const GPIO_J4: hal_ll_pin_name_t = 0x94;
#[cfg(feature = "gpioj_5")]
pub const GPIO_J5: hal_ll_pin_name_t = 0x95;
#[cfg(feature = "gpioj_6")]
pub const GPIO_J6: hal_ll_pin_name_t = 0x96;
#[cfg(feature = "gpioj_7")]
pub const GPIO_J7: hal_ll_pin_name_t = 0x97;
#[cfg(feature = "gpioj_8")]
pub const GPIO_J8: hal_ll_pin_name_t = 0x98;
#[cfg(feature = "gpioj_9")]
pub const GPIO_J9: hal_ll_pin_name_t = 0x99;
#[cfg(feature = "gpioj_10")]
pub const GPIO_J10: hal_ll_pin_name_t = 0x9A;
#[cfg(feature = "gpioj_11")]
pub const GPIO_J11: hal_ll_pin_name_t = 0x9B;
#[cfg(feature = "gpioj_12")]
pub const GPIO_J12: hal_ll_pin_name_t = 0x9C;
#[cfg(feature = "gpioj_13")]
pub const GPIO_J13: hal_ll_pin_name_t = 0x9D;
#[cfg(feature = "gpioj_14")]
pub const GPIO_J14: hal_ll_pin_name_t = 0x9E;
#[cfg(feature = "gpioj_15")]
pub const GPIO_J15: hal_ll_pin_name_t = 0x9F;

#[cfg(feature = "gpiok_0")]
pub const GPIO_K0: hal_ll_pin_name_t = 0xA0;
#[cfg(feature = "gpiok_1")]
pub const GPIO_K1: hal_ll_pin_name_t = 0xA1;
#[cfg(feature = "gpiok_2")]
pub const GPIO_K2: hal_ll_pin_name_t = 0xA2;
#[cfg(feature = "gpiok_3")]
pub const GPIO_K3: hal_ll_pin_name_t = 0xA3;
#[cfg(feature = "gpiok_4")]
pub const GPIO_K4: hal_ll_pin_name_t = 0xA4;
#[cfg(feature = "gpiok_5")]
pub const GPIO_K5: hal_ll_pin_name_t = 0xA5;
#[cfg(feature = "gpiok_6")]
pub const GPIO_K6: hal_ll_pin_name_t = 0xA6;
#[cfg(feature = "gpiok_7")]
pub const GPIO_K7: hal_ll_pin_name_t = 0xA7;
#[cfg(feature = "gpiok_8")]
pub const GPIO_K8: hal_ll_pin_name_t = 0xA8;
#[cfg(feature = "gpiok_9")]
pub const GPIO_K9: hal_ll_pin_name_t = 0xA9;
#[cfg(feature = "gpiok_10")]
pub const GPIO_K10: hal_ll_pin_name_t = 0xAA;
#[cfg(feature = "gpiok_11")]
pub const GPIO_K11: hal_ll_pin_name_t = 0xAB;
#[cfg(feature = "gpiok_12")]
pub const GPIO_K12: hal_ll_pin_name_t = 0xAC;
#[cfg(feature = "gpiok_13")]
pub const GPIO_K13: hal_ll_pin_name_t = 0xAD;
#[cfg(feature = "gpiok_14")]
pub const GPIO_K14: hal_ll_pin_name_t = 0xAE;
#[cfg(feature = "gpiok_15")]
pub const GPIO_K15: hal_ll_pin_name_t = 0xAF;