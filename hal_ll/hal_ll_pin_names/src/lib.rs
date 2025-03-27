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

#![no_std]

pub use hal_ll_target_names::*;
pub use mcu::*;

pub const HAL_LL_PIN_NC : hal_ll_pin_name_t = 0xFF;
pub const HAL_LL_PORT_NC : hal_ll_pin_name_t = 0xFF;
pub const HAL_LL_MODULE_ERROR: u32 = 0xFFFF_FFFF;

pub const GPIO_A0: hal_ll_pin_name_t = 0x00;
pub const GPIO_A1: hal_ll_pin_name_t = 0x01;
pub const GPIO_A2: hal_ll_pin_name_t = 0x02;
pub const GPIO_A3: hal_ll_pin_name_t = 0x03;
pub const GPIO_A4: hal_ll_pin_name_t = 0x04;
pub const GPIO_A5: hal_ll_pin_name_t = 0x05;
pub const GPIO_A6: hal_ll_pin_name_t = 0x06;
pub const GPIO_A7: hal_ll_pin_name_t = 0x07;
pub const GPIO_A8: hal_ll_pin_name_t = 0x08;
pub const GPIO_A9: hal_ll_pin_name_t = 0x09;
pub const GPIO_A10: hal_ll_pin_name_t = 0x0A;
pub const GPIO_A11: hal_ll_pin_name_t = 0x0B;
pub const GPIO_A12: hal_ll_pin_name_t = 0x0C;
pub const GPIO_A13: hal_ll_pin_name_t = 0x0D;
pub const GPIO_A14: hal_ll_pin_name_t = 0x0E;
pub const GPIO_A15: hal_ll_pin_name_t = 0x0F;

pub const GPIO_B0: hal_ll_pin_name_t = 0x10;
pub const GPIO_B1: hal_ll_pin_name_t = 0x11;
pub const GPIO_B2: hal_ll_pin_name_t = 0x12;
pub const GPIO_B3: hal_ll_pin_name_t = 0x13;
pub const GPIO_B4: hal_ll_pin_name_t = 0x14;
pub const GPIO_B5: hal_ll_pin_name_t = 0x15;
pub const GPIO_B6: hal_ll_pin_name_t = 0x16;
pub const GPIO_B7: hal_ll_pin_name_t = 0x17;
pub const GPIO_B8: hal_ll_pin_name_t = 0x18;
pub const GPIO_B9: hal_ll_pin_name_t = 0x19;
pub const GPIO_B10: hal_ll_pin_name_t = 0x1A;
pub const GPIO_B11: hal_ll_pin_name_t = 0x1B;
pub const GPIO_B12: hal_ll_pin_name_t = 0x1C;
pub const GPIO_B13: hal_ll_pin_name_t = 0x1D;
pub const GPIO_B14: hal_ll_pin_name_t = 0x1E;
pub const GPIO_B15: hal_ll_pin_name_t = 0x1F;

pub const GPIO_C0: hal_ll_pin_name_t = 0x20;
pub const GPIO_C1: hal_ll_pin_name_t = 0x21;
pub const GPIO_C2: hal_ll_pin_name_t = 0x22;
pub const GPIO_C3: hal_ll_pin_name_t = 0x23;
pub const GPIO_C4: hal_ll_pin_name_t = 0x24;
pub const GPIO_C5: hal_ll_pin_name_t = 0x25;
pub const GPIO_C6: hal_ll_pin_name_t = 0x26;
pub const GPIO_C7: hal_ll_pin_name_t = 0x27;
pub const GPIO_C8: hal_ll_pin_name_t = 0x28;
pub const GPIO_C9: hal_ll_pin_name_t = 0x29;
pub const GPIO_C10: hal_ll_pin_name_t = 0x2A;
pub const GPIO_C11: hal_ll_pin_name_t = 0x2B;
pub const GPIO_C12: hal_ll_pin_name_t = 0x2C;
pub const GPIO_C13: hal_ll_pin_name_t = 0x2D;
pub const GPIO_C14: hal_ll_pin_name_t = 0x2E;
pub const GPIO_C15: hal_ll_pin_name_t = 0x2F;

pub const GPIO_D0: hal_ll_pin_name_t = 0x30;
pub const GPIO_D1: hal_ll_pin_name_t = 0x31;
pub const GPIO_D2: hal_ll_pin_name_t = 0x32;
pub const GPIO_D3: hal_ll_pin_name_t = 0x33;
pub const GPIO_D4: hal_ll_pin_name_t = 0x34;
pub const GPIO_D5: hal_ll_pin_name_t = 0x35;
pub const GPIO_D6: hal_ll_pin_name_t = 0x36;
pub const GPIO_D7: hal_ll_pin_name_t = 0x37;
pub const GPIO_D8: hal_ll_pin_name_t = 0x38;
pub const GPIO_D9: hal_ll_pin_name_t = 0x39;
pub const GPIO_D10: hal_ll_pin_name_t = 0x3A;
pub const GPIO_D11: hal_ll_pin_name_t = 0x3B;
pub const GPIO_D12: hal_ll_pin_name_t = 0x3C;
pub const GPIO_D13: hal_ll_pin_name_t = 0x3D;
pub const GPIO_D14: hal_ll_pin_name_t = 0x3E;
pub const GPIO_D15: hal_ll_pin_name_t = 0x3F;

pub const GPIO_E0: hal_ll_pin_name_t = 0x40;
pub const GPIO_E1: hal_ll_pin_name_t = 0x41;
pub const GPIO_E2: hal_ll_pin_name_t = 0x42;
pub const GPIO_E3: hal_ll_pin_name_t = 0x43;
pub const GPIO_E4: hal_ll_pin_name_t = 0x44;
pub const GPIO_E5: hal_ll_pin_name_t = 0x45;
pub const GPIO_E6: hal_ll_pin_name_t = 0x46;
pub const GPIO_E7: hal_ll_pin_name_t = 0x47;
pub const GPIO_E8: hal_ll_pin_name_t = 0x48;
pub const GPIO_E9: hal_ll_pin_name_t = 0x49;
pub const GPIO_E10: hal_ll_pin_name_t = 0x4A;
pub const GPIO_E11: hal_ll_pin_name_t = 0x4B;
pub const GPIO_E12: hal_ll_pin_name_t = 0x4C;
pub const GPIO_E13: hal_ll_pin_name_t = 0x4D;
pub const GPIO_E14: hal_ll_pin_name_t = 0x4E;
pub const GPIO_E15: hal_ll_pin_name_t = 0x4F;

pub const GPIO_F0: hal_ll_pin_name_t = 0x50;
pub const GPIO_F1: hal_ll_pin_name_t = 0x51;
pub const GPIO_F2: hal_ll_pin_name_t = 0x52;
pub const GPIO_F3: hal_ll_pin_name_t = 0x53;
pub const GPIO_F4: hal_ll_pin_name_t = 0x54;
pub const GPIO_F5: hal_ll_pin_name_t = 0x55;
pub const GPIO_F6: hal_ll_pin_name_t = 0x56;
pub const GPIO_F7: hal_ll_pin_name_t = 0x57;
pub const GPIO_F8: hal_ll_pin_name_t = 0x58;
pub const GPIO_F9: hal_ll_pin_name_t = 0x59;
pub const GPIO_F10: hal_ll_pin_name_t = 0x5A;
pub const GPIO_F11: hal_ll_pin_name_t = 0x5B;
pub const GPIO_F12: hal_ll_pin_name_t = 0x5C;
pub const GPIO_F13: hal_ll_pin_name_t = 0x5D;
pub const GPIO_F14: hal_ll_pin_name_t = 0x5E;
pub const GPIO_F15: hal_ll_pin_name_t = 0x5F;

pub const GPIO_G0: hal_ll_pin_name_t = 0x60;
pub const GPIO_G1: hal_ll_pin_name_t = 0x61;
pub const GPIO_G2: hal_ll_pin_name_t = 0x62;
pub const GPIO_G3: hal_ll_pin_name_t = 0x63;
pub const GPIO_G4: hal_ll_pin_name_t = 0x64;
pub const GPIO_G5: hal_ll_pin_name_t = 0x65;
pub const GPIO_G6: hal_ll_pin_name_t = 0x66;
pub const GPIO_G7: hal_ll_pin_name_t = 0x67;
pub const GPIO_G8: hal_ll_pin_name_t = 0x68;
pub const GPIO_G9: hal_ll_pin_name_t = 0x69;
pub const GPIO_G10: hal_ll_pin_name_t = 0x6A;
pub const GPIO_G11: hal_ll_pin_name_t = 0x6B;
pub const GPIO_G12: hal_ll_pin_name_t = 0x6C;
pub const GPIO_G13: hal_ll_pin_name_t = 0x6D;
pub const GPIO_G14: hal_ll_pin_name_t = 0x6E;
pub const GPIO_G15: hal_ll_pin_name_t = 0x6F;

pub const GPIO_H0: hal_ll_pin_name_t = 0x70;
pub const GPIO_H1: hal_ll_pin_name_t = 0x71;
pub const GPIO_H2: hal_ll_pin_name_t = 0x72;
pub const GPIO_H3: hal_ll_pin_name_t = 0x73;
pub const GPIO_H4: hal_ll_pin_name_t = 0x74;
pub const GPIO_H5: hal_ll_pin_name_t = 0x75;
pub const GPIO_H6: hal_ll_pin_name_t = 0x76;
pub const GPIO_H7: hal_ll_pin_name_t = 0x77;
pub const GPIO_H8: hal_ll_pin_name_t = 0x78;
pub const GPIO_H9: hal_ll_pin_name_t = 0x79;
pub const GPIO_H10: hal_ll_pin_name_t = 0x7A;
pub const GPIO_H11: hal_ll_pin_name_t = 0x7B;
pub const GPIO_H12: hal_ll_pin_name_t = 0x7C;
pub const GPIO_H13: hal_ll_pin_name_t = 0x7D;
pub const GPIO_H14: hal_ll_pin_name_t = 0x7E;
pub const GPIO_H15: hal_ll_pin_name_t = 0x7F;

pub const GPIO_I0: hal_ll_pin_name_t = 0x80;
pub const GPIO_I1: hal_ll_pin_name_t = 0x81;
pub const GPIO_I2: hal_ll_pin_name_t = 0x82;
pub const GPIO_I3: hal_ll_pin_name_t = 0x83;
pub const GPIO_I4: hal_ll_pin_name_t = 0x84;
pub const GPIO_I5: hal_ll_pin_name_t = 0x85;
pub const GPIO_I6: hal_ll_pin_name_t = 0x86;
pub const GPIO_I7: hal_ll_pin_name_t = 0x87;
pub const GPIO_I8: hal_ll_pin_name_t = 0x88;
pub const GPIO_I9: hal_ll_pin_name_t = 0x89;
pub const GPIO_I10: hal_ll_pin_name_t = 0x8A;
pub const GPIO_I11: hal_ll_pin_name_t = 0x8B;
pub const GPIO_I12: hal_ll_pin_name_t = 0x8C;
pub const GPIO_I13: hal_ll_pin_name_t = 0x8D;
pub const GPIO_I14: hal_ll_pin_name_t = 0x8E;
pub const GPIO_I15: hal_ll_pin_name_t = 0x8F;

pub const GPIO_J0: hal_ll_pin_name_t = 0x90;
pub const GPIO_J1: hal_ll_pin_name_t = 0x91;
pub const GPIO_J2: hal_ll_pin_name_t = 0x92;
pub const GPIO_J3: hal_ll_pin_name_t = 0x93;
pub const GPIO_J4: hal_ll_pin_name_t = 0x94;
pub const GPIO_J5: hal_ll_pin_name_t = 0x95;
pub const GPIO_J6: hal_ll_pin_name_t = 0x96;
pub const GPIO_J7: hal_ll_pin_name_t = 0x97;
pub const GPIO_J8: hal_ll_pin_name_t = 0x98;
pub const GPIO_J9: hal_ll_pin_name_t = 0x99;
pub const GPIO_J10: hal_ll_pin_name_t = 0x9A;
pub const GPIO_J11: hal_ll_pin_name_t = 0x9B;
pub const GPIO_J12: hal_ll_pin_name_t = 0x9C;
pub const GPIO_J13: hal_ll_pin_name_t = 0x9D;
pub const GPIO_J14: hal_ll_pin_name_t = 0x9E;
pub const GPIO_J15: hal_ll_pin_name_t = 0x9F;

pub const GPIO_K0: hal_ll_pin_name_t = 0xA0;
pub const GPIO_K1: hal_ll_pin_name_t = 0xA1;
pub const GPIO_K2: hal_ll_pin_name_t = 0xA2;
pub const GPIO_K3: hal_ll_pin_name_t = 0xA3;
pub const GPIO_K4: hal_ll_pin_name_t = 0xA4;
pub const GPIO_K5: hal_ll_pin_name_t = 0xA5;
pub const GPIO_K6: hal_ll_pin_name_t = 0xA6;
pub const GPIO_K7: hal_ll_pin_name_t = 0xA7;
pub const GPIO_K8: hal_ll_pin_name_t = 0xA8;
pub const GPIO_K9: hal_ll_pin_name_t = 0xA9;
pub const GPIO_K10: hal_ll_pin_name_t = 0xAA;
pub const GPIO_K11: hal_ll_pin_name_t = 0xAB;
pub const GPIO_K12: hal_ll_pin_name_t = 0xAC;
pub const GPIO_K13: hal_ll_pin_name_t = 0xAD;
pub const GPIO_K14: hal_ll_pin_name_t = 0xAE;
pub const GPIO_K15: hal_ll_pin_name_t = 0xAF;

pub const GPIO_PORT_A: hal_ll_port_name_t = 0x00;
pub const GPIO_PORT_B: hal_ll_port_name_t = 0x01;
pub const GPIO_PORT_C: hal_ll_port_name_t = 0x02;
pub const GPIO_PORT_D: hal_ll_port_name_t = 0x03;
pub const GPIO_PORT_E: hal_ll_port_name_t = 0x04;
pub const GPIO_PORT_F: hal_ll_port_name_t = 0x05;
pub const GPIO_PORT_G: hal_ll_port_name_t = 0x06;
pub const GPIO_PORT_H: hal_ll_port_name_t = 0x07;
pub const GPIO_PORT_I: hal_ll_port_name_t = 0x08;
pub const GPIO_PORT_J: hal_ll_port_name_t = 0x09;
pub const GPIO_PORT_K: hal_ll_port_name_t = 0x0A;