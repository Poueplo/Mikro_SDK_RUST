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
#![allow(non_snake_case)]

use core::arch::asm;

//todo move to core_header
const FOSC_KHZ_VALUE: u32 = 180000;

//todo move in delay_parameters
fn getClockValue(_clock: u32) -> u32 {
    _clock / 1000 / 6
}

pub fn Delay_Cyc(mut cycle_num: u32)
{
    unsafe{
        asm! (
            "1:",
            "   subs {cycle_num}, {cycle_num}, #1",
            "   nop",
            "   bne 1b",
            cycle_num = inout(reg) cycle_num,
        );
    }
}

pub fn Delay_us(time_us: u32) {
    Delay_Cyc( ( time_us * getClockValue( FOSC_KHZ_VALUE ) ) - ( getClockValue( FOSC_KHZ_VALUE ) / 2 ) );
}

pub fn Delay_ms(time_ms: u32)
{
    Delay_Cyc( time_ms * 1000 * getClockValue( FOSC_KHZ_VALUE ) );
}

pub fn Delay_Advanced_ms(time_ms: u32, Current_Fosc_kHz: u32 )
{
    Delay_Cyc( time_ms * 1000 * getClockValue( Current_Fosc_kHz ) );
}

pub fn Delay_1us()
{
    Delay_us( 1 );
}

pub fn Delay_5us()
{
    Delay_us( 5 );
}

pub fn Delay_6us()
{
    Delay_us( 6 );
}

pub fn Delay_9us()
{
    Delay_us( 9 );
}

pub fn Delay_10us()
{
    Delay_us( 10 );
}

pub fn Delay_22us()
{
    Delay_us( 22 );
}

pub fn Delay_50us()
{
    Delay_us( 50 );
}

pub fn Delay_55us()
{
    Delay_us( 55 );
}

pub fn Delay_60us()
{
    Delay_us( 60 );
}

pub fn Delay_64us()
{
    Delay_us( 64 );
}

pub fn Delay_70us()
{
    Delay_us( 70 );
}

pub fn Delay_80us()
{
    Delay_us( 78 );
}

pub fn Delay_410us()
{
    Delay_us( 410 );
}

pub fn Delay_480us()
{
    Delay_us( 480 );
}

pub fn Delay_500us()
{
    Delay_us( 498 );
}

pub fn Delay_5500us()
{
    Delay_us( 5500 );
}

pub fn Delay_1ms()
{
    Delay_ms( 1 );
}

pub fn Delay_5ms()
{
    Delay_ms( 5 );
}

pub fn Delay_8ms()
{
    Delay_ms( 8 );
}

pub fn Delay_10ms()
{
    Delay_ms( 10 );
}

pub fn Delay_100ms()//too much for some reasons
{
    Delay_ms( 100 );
}

pub fn Delay_1sec()//too much for some reasons
{
    Delay_ms( 1000 );
}