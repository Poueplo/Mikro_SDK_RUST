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
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub struct ring_buf8_t<const capacity : usize> {
    buffer : [u8; capacity],
    size : usize,
    head : usize,
    tail : usize,
}

impl<const capacity : usize> ring_buf8_t<capacity> {
    pub const fn init() -> Self {
        Self {
            buffer: [0; capacity],
            size: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, data_ : u8) -> Result<(), ()> {
        if self.size == (capacity + 1) {
            return Err(());
        }

        self.buffer[self.head] = data_;
        self.head = (self.head + 1) % capacity;
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u8, ()> {
        let result : u8;
        if self.size <= 0 {
            return Err(());
        }

        result = self.buffer[self.tail];
        self.tail = (self.tail + 1) % capacity;
        self.size -= 1;

        Ok(result)
    }

    pub fn is_empty(&mut self) -> bool {
        self.size == 0
    }

    pub fn is_full(&mut self) -> bool {
        self.size == capacity
    }

    pub fn get_size(&mut self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = 0;
        self.tail = 0;
    }
}