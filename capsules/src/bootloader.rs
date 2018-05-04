//! Implements the Tock bootloader.

use core::cell::Cell;
use core::cmp;
use kernel::common::take_cell::TakeCell;
use kernel::hil;

extern crate tockloader_proto;

// Main buffer that commands are received into and sent from.
pub static mut BUF: [u8; 600] = [0; 600];

static CRC32_TABLE
    : [u32; 256]
    = [   0x0u32,
          0x77073096u32,
          0xee0e612cu32,
          0x990951bau32,
          0x76dc419u32,
          0x706af48fu32,
          0xe963a535u32,
          0x9e6495a3u32,
          0xedb8832u32,
          0x79dcb8a4u32,
          0xe0d5e91eu32,
          0x97d2d988u32,
          0x9b64c2bu32,
          0x7eb17cbdu32,
          0xe7b82d07u32,
          0x90bf1d91u32,
          0x1db71064u32,
          0x6ab020f2u32,
          0xf3b97148u32,
          0x84be41deu32,
          0x1adad47du32,
          0x6ddde4ebu32,
          0xf4d4b551u32,
          0x83d385c7u32,
          0x136c9856u32,
          0x646ba8c0u32,
          0xfd62f97au32,
          0x8a65c9ecu32,
          0x14015c4fu32,
          0x63066cd9u32,
          0xfa0f3d63u32,
          0x8d080df5u32,
          0x3b6e20c8u32,
          0x4c69105eu32,
          0xd56041e4u32,
          0xa2677172u32,
          0x3c03e4d1u32,
          0x4b04d447u32,
          0xd20d85fdu32,
          0xa50ab56bu32,
          0x35b5a8fau32,
          0x42b2986cu32,
          0xdbbbc9d6u32,
          0xacbcf940u32,
          0x32d86ce3u32,
          0x45df5c75u32,
          0xdcd60dcfu32,
          0xabd13d59u32,
          0x26d930acu32,
          0x51de003au32,
          0xc8d75180u32,
          0xbfd06116u32,
          0x21b4f4b5u32,
          0x56b3c423u32,
          0xcfba9599u32,
          0xb8bda50fu32,
          0x2802b89eu32,
          0x5f058808u32,
          0xc60cd9b2u32,
          0xb10be924u32,
          0x2f6f7c87u32,
          0x58684c11u32,
          0xc1611dabu32,
          0xb6662d3du32,
          0x76dc4190u32,
          0x1db7106u32,
          0x98d220bcu32,
          0xefd5102au32,
          0x71b18589u32,
          0x6b6b51fu32,
          0x9fbfe4a5u32,
          0xe8b8d433u32,
          0x7807c9a2u32,
          0xf00f934u32,
          0x9609a88eu32,
          0xe10e9818u32,
          0x7f6a0dbbu32,
          0x86d3d2du32,
          0x91646c97u32,
          0xe6635c01u32,
          0x6b6b51f4u32,
          0x1c6c6162u32,
          0x856530d8u32,
          0xf262004eu32,
          0x6c0695edu32,
          0x1b01a57bu32,
          0x8208f4c1u32,
          0xf50fc457u32,
          0x65b0d9c6u32,
          0x12b7e950u32,
          0x8bbeb8eau32,
          0xfcb9887cu32,
          0x62dd1ddfu32,
          0x15da2d49u32,
          0x8cd37cf3u32,
          0xfbd44c65u32,
          0x4db26158u32,
          0x3ab551ceu32,
          0xa3bc0074u32,
          0xd4bb30e2u32,
          0x4adfa541u32,
          0x3dd895d7u32,
          0xa4d1c46du32,
          0xd3d6f4fbu32,
          0x4369e96au32,
          0x346ed9fcu32,
          0xad678846u32,
          0xda60b8d0u32,
          0x44042d73u32,
          0x33031de5u32,
          0xaa0a4c5fu32,
          0xdd0d7cc9u32,
          0x5005713cu32,
          0x270241aau32,
          0xbe0b1010u32,
          0xc90c2086u32,
          0x5768b525u32,
          0x206f85b3u32,
          0xb966d409u32,
          0xce61e49fu32,
          0x5edef90eu32,
          0x29d9c998u32,
          0xb0d09822u32,
          0xc7d7a8b4u32,
          0x59b33d17u32,
          0x2eb40d81u32,
          0xb7bd5c3bu32,
          0xc0ba6cadu32,
          0xedb88320u32,
          0x9abfb3b6u32,
          0x3b6e20cu32,
          0x74b1d29au32,
          0xead54739u32,
          0x9dd277afu32,
          0x4db2615u32,
          0x73dc1683u32,
          0xe3630b12u32,
          0x94643b84u32,
          0xd6d6a3eu32,
          0x7a6a5aa8u32,
          0xe40ecf0bu32,
          0x9309ff9du32,
          0xa00ae27u32,
          0x7d079eb1u32,
          0xf00f9344u32,
          0x8708a3d2u32,
          0x1e01f268u32,
          0x6906c2feu32,
          0xf762575du32,
          0x806567cbu32,
          0x196c3671u32,
          0x6e6b06e7u32,
          0xfed41b76u32,
          0x89d32be0u32,
          0x10da7a5au32,
          0x67dd4accu32,
          0xf9b9df6fu32,
          0x8ebeeff9u32,
          0x17b7be43u32,
          0x60b08ed5u32,
          0xd6d6a3e8u32,
          0xa1d1937eu32,
          0x38d8c2c4u32,
          0x4fdff252u32,
          0xd1bb67f1u32,
          0xa6bc5767u32,
          0x3fb506ddu32,
          0x48b2364bu32,
          0xd80d2bdau32,
          0xaf0a1b4cu32,
          0x36034af6u32,
          0x41047a60u32,
          0xdf60efc3u32,
          0xa867df55u32,
          0x316e8eefu32,
          0x4669be79u32,
          0xcb61b38cu32,
          0xbc66831au32,
          0x256fd2a0u32,
          0x5268e236u32,
          0xcc0c7795u32,
          0xbb0b4703u32,
          0x220216b9u32,
          0x5505262fu32,
          0xc5ba3bbeu32,
          0xb2bd0b28u32,
          0x2bb45a92u32,
          0x5cb36a04u32,
          0xc2d7ffa7u32,
          0xb5d0cf31u32,
          0x2cd99e8bu32,
          0x5bdeae1du32,
          0x9b64c2b0u32,
          0xec63f226u32,
          0x756aa39cu32,
          0x26d930au32,
          0x9c0906a9u32,
          0xeb0e363fu32,
          0x72076785u32,
          0x5005713u32,
          0x95bf4a82u32,
          0xe2b87a14u32,
          0x7bb12baeu32,
          0xcb61b38u32,
          0x92d28e9bu32,
          0xe5d5be0du32,
          0x7cdcefb7u32,
          0xbdbdf21u32,
          0x86d3d2d4u32,
          0xf1d4e242u32,
          0x68ddb3f8u32,
          0x1fda836eu32,
          0x81be16cdu32,
          0xf6b9265bu32,
          0x6fb077e1u32,
          0x18b74777u32,
          0x88085ae6u32,
          0xff0f6a70u32,
          0x66063bcau32,
          0x11010b5cu32,
          0x8f659effu32,
          0xf862ae69u32,
          0x616bffd3u32,
          0x166ccf45u32,
          0xa00ae278u32,
          0xd70dd2eeu32,
          0x4e048354u32,
          0x3903b3c2u32,
          0xa7672661u32,
          0xd06016f7u32,
          0x4969474du32,
          0x3e6e77dbu32,
          0xaed16a4au32,
          0xd9d65adcu32,
          0x40df0b66u32,
          0x37d83bf0u32,
          0xa9bcae53u32,
          0xdebb9ec5u32,
          0x47b2cf7fu32,
          0x30b5ffe9u32,
          0xbdbdf21cu32,
          0xcabac28au32,
          0x53b39330u32,
          0x24b4a3a6u32,
          0xbad03605u32,
          0xcdd70693u32,
          0x54de5729u32,
          0x23d967bfu32,
          0xb3667a2eu32,
          0xc4614ab8u32,
          0x5d681b02u32,
          0x2a6f2b94u32,
          0xb40bbe37u32,
          0xc30c8ea1u32,
          0x5a05df1bu32,
          0x2d02ef8du32
      ];


const ESCAPE_CHAR: u8 = 0xFC;

const RES_PONG: u8 = 0x11;
const RES_INTERNAL_ERROR: u8 = 0x13;
const RES_BADARGS: u8 = 0x14;
const RES_OK: u8 = 0x15;
const RES_UNKNOWN: u8 = 0x16;
const RES_READ_RANGE: u8 = 0x20;
const RES_GET_ATTR: u8 = 0x22;
const RES_CRCIF: u8 = 0x23;
const RES_INFO: u8 = 0x25;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Idle,
    Info,
    ErasePage,
    GetAttribute{index: u8},
    SetAttribute{index: u8},
    WriteFlashPage,
    ReadRange{address: u32, length: u16, remaining_length: u16},
    Crc{address: u32, remaining_length: u32, crc: u32},
}

pub struct Bootloader<'a, U: hil::uart::UARTAdvanced + 'a, F: hil::flash::Flash + 'static, G: hil::gpio::Pin + 'a> {
    uart: &'a U,
    flash: &'a F,
    select_pin: &'a G,
    led: &'a G,
    dpin: &'a G,
    /// Buffer correctly sized for the underlying flash page size.
    page_buffer: TakeCell<'static, F::Page>,
    buffer: TakeCell<'static, [u8]>,
    state: Cell<State>,
}

impl<'a, U: hil::uart::UARTAdvanced + 'a, F: hil::flash::Flash + 'a, G: hil::gpio::Pin + 'a> Bootloader<'a, U, F, G> {
    pub fn new(uart: &'a U, flash: &'a F, select_pin: &'a G, led: &'a G, dpin: &'a G,
               page_buffer: &'static mut F::Page,
               buffer: &'static mut [u8])
               -> Bootloader<'a, U, F, G> {
        Bootloader {
            uart: uart,
            flash: flash,
            select_pin: select_pin,
            led: led,
            dpin: dpin,
            page_buffer: TakeCell::new(page_buffer),
            buffer: TakeCell::new(buffer),
            state: Cell::new(State::Idle),
        }
    }

    pub fn initialize(&self) {

        // Setup UART and start listening.
        self.uart.init(hil::uart::UARTParams {
            baud_rate: 115200,
            stop_bits: hil::uart::StopBits::One,
            parity: hil::uart::Parity::None,
            hw_flow_control: false,
        });



        // // self.select_pin.enable();
        // self.select_pin.make_input();



        // // Check the select pin to see if we should enter bootloader mode.
        // let mut samples = 10000;
        // let mut active = 0;
        // let mut inactive = 0;
        // while samples > 0 {
        //     if self.select_pin.read() == false {
        //         active += 1;
        //     } else {
        //         inactive += 1;
        //     }
        //     samples -= 1;
        // }

        // if active > inactive {
            // Looks like we do want bootloader mode.





            self.buffer.take().map(|buffer| {
                self.dpin.toggle();
                self.led.toggle();
                self.uart.receive_automatic(buffer, 250);
                // self.uart.receive(buffer, 2);
                // buffer[0] = 97;
                // buffer[1] = 98;
                // buffer[2] = 100;
                // buffer[3] = 105;
                // buffer[4] = 110;
                // self.uart.transmit(buffer, 5);
            });




        // } else {
        //     // Jump to the kernel and start the real code.
        // }


    }


    // Helper function for sending single byte responses.
    fn send_response (&self, response: u8) {
        self.buffer.take().map(|buffer| {
            buffer[0] = ESCAPE_CHAR;
            buffer[1] = response;
            self.uart.transmit(buffer, 2);
        });
    }
}

impl<'a, U: hil::uart::UARTAdvanced + 'a, F: hil::flash::Flash + 'a, G: hil::gpio::Pin + 'a> hil::uart::Client for Bootloader<'a, U, F, G> {
    fn transmit_complete(&self, buffer: &'static mut [u8], error: hil::uart::Error) {
        if error != hil::uart::Error::CommandComplete {
            // self.led.clear();
        } else {

            match self.state.get() {

                // Check if there is more to be read, and if so, read it and
                // send it.
                State::ReadRange{address, length: _, remaining_length} => {
                    // We have sent some of the read range to the client.
                    // We are either done, or need to setup the next read.
                    if remaining_length == 0 {
                        self.state.set(State::Idle);
                        self.uart.receive_automatic(buffer, 250);

                    } else {
                        self.buffer.replace(buffer);
                        self.page_buffer.take().map(move |page| {
                            let page_size = page.as_mut().len();
                            self.flash.read_page(address as usize / page_size, page);
                        });
                    }
                }

                _ => {
                    self.uart.receive_automatic(buffer, 250);
                }
            }
        }

    }

    fn receive_complete(&self,
                        buffer: &'static mut [u8],
                        rx_len: usize,
                        error: hil::uart::Error) {


        if error != hil::uart::Error::CommandComplete {
            // self.led.clear();
            return
        }

        // Tool to parse incoming bootloader messages.
        let mut decoder = tockloader_proto::CommandDecoder::new();
        // Whether we want to reset the position in the buffer in the
        // decoder.
        let mut need_reset = false;

        // Loop through the buffer and pass it to the decoder.
        for i in 0..rx_len {
            match decoder.receive(buffer[i]) {
                Ok(None) => {},
                Ok(Some(tockloader_proto::Command::Ping)) => {

                    self.buffer.replace(buffer);
                    self.send_response(RES_PONG);
                    break;
                }
                Ok(Some(tockloader_proto::Command::Reset)) => {
                    need_reset = true;
                    self.buffer.replace(buffer);
                    break;
                }
                Ok(Some(tockloader_proto::Command::Info)) => {
                    self.state.set(State::Info);
                    self.buffer.replace(buffer);
                    self.page_buffer.take().map(move |page| {
                        self.flash.read_page(2, page);
                    });
                    break;
                }
                Ok(Some(tockloader_proto::Command::ReadRange{address, length})) => {
                    self.state.set(State::ReadRange{address, length, remaining_length: length});
                    self.buffer.replace(buffer);
                    self.page_buffer.take().map(move |page| {
                        let page_size = page.as_mut().len();
                        self.flash.read_page(address as usize / page_size, page);
                    });
                    break;
                }
                Ok(Some(tockloader_proto::Command::WritePage{address, data})) => {
                    self.page_buffer.take().map(move |page| {
                        let page_size = page.as_mut().len();
                        if page_size != data.len() {
                            // Error if we didn't get exactly a page of data
                            // to write to flash.
                            buffer[0] = ESCAPE_CHAR;
                            buffer[1] = RES_BADARGS;
                            self.page_buffer.replace(page);
                            self.state.set(State::Idle);
                            self.uart.transmit(buffer, 2);
                        } else {
                            // Otherwise copy into page buffer and write to
                            // flash.
                            for i in 0..page_size {
                                page.as_mut()[i] = data[i];
                            }
                            self.state.set(State::WriteFlashPage);
                            self.buffer.replace(buffer);
                            self.flash.write_page(address as usize / page_size, page);
                        }
                    });
                    break;
                }
                Ok(Some(tockloader_proto::Command::ErasePage{address})) => {
                    self.state.set(State::ErasePage);
                    self.buffer.replace(buffer);
                    let page_size = self.page_buffer.map_or(512, |page| { page.as_mut().len() });
                    self.flash.erase_page(address as usize / page_size);
                    break;
                }
                Ok(Some(tockloader_proto::Command::CrcIntFlash{address, length})) => {
                    self.state.set(State::Crc{address, remaining_length: length, crc: 0xFFFFFFFF});
                    self.buffer.replace(buffer);
                    self.page_buffer.take().map(move |page| {
                        let page_size = page.as_mut().len();
                        self.flash.read_page(address as usize / page_size, page);
                    });
                    break;
                }
                Ok(Some(tockloader_proto::Command::GetAttr{index})) => {
                    self.state.set(State::GetAttribute{index: index});
                    self.buffer.replace(buffer);
                    self.page_buffer.take().map(move |page| {
                        self.flash.read_page(3 + (index as usize / 8), page);
                    });
                    break;
                }
                Ok(Some(tockloader_proto::Command::SetAttr{index, key, value})) => {
                    self.state.set(State::SetAttribute{index});

                    // Copy the key and value into the buffer so it can be added
                    // to the page buffer when needed.
                    for i in 0..8 {
                        buffer[i] = key[i];
                    }
                    buffer[8] = value.len() as u8;
                    for i in 0..55 {
                        // Copy in the value, otherwise clear to zero.
                        if i < value.len() {
                            buffer[9 + i] = value[i];
                        } else {
                            buffer[9+i] = 0;
                        }
                    }
                    self.buffer.replace(buffer);

                    // Initiate things by reading the correct flash page that
                    // needs to be updated.
                    self.page_buffer.take().map(move |page| {
                        self.flash.read_page(3 + (index as usize / 8), page);
                    });
                    break;
                }
                Ok(Some(_)) => {
                    self.send_response(RES_UNKNOWN);
                    break;
                }
                Err(_) => {
                    self.send_response(RES_INTERNAL_ERROR);
                    break;
                }
            };
        }

        // Artifact of the original implementation of the bootloader protocol
        // is the need to reset the pointer internal to the bootloader receive
        // state machine.
        if need_reset {
            decoder.reset();

            self.buffer.take().map(|buffer| {
                self.uart.receive_automatic(buffer, 250);
            });

        }
    }
}

impl<'a, U: hil::uart::UARTAdvanced + 'a, F: hil::flash::Flash + 'a, G: hil::gpio::Pin + 'a> hil::flash::Client<F> for Bootloader<'a, U, F, G> {
    fn read_complete(&self, pagebuffer: &'static mut F::Page, _error: hil::flash::Error) {


        match self.state.get() {

            // We just read the bootloader info page (page 2). Extract the
            // version and generate a response JSON blob.
            State::Info => {

                self.state.set(State::Idle);
                self.buffer.take().map(move |buffer| {
                    buffer[0] = ESCAPE_CHAR;
                    buffer[1] = RES_INFO;

                    // "{\"version\":\"%s\", \"name\":\"Tock Bootloader\"}"

                    // Version string is at most 8 bytes long, and starts
                    // at index 14 in the bootloader page.
                    for i in 0..8 {
                        let b = pagebuffer.as_mut()[i+14];
                        if b == 0 {
                            break;
                        }
                        buffer[i+2] = b;
                    }
                    for i in 10..195 {
                        buffer[i] = 0;
                    }

                    self.page_buffer.replace(pagebuffer);
                    self.uart.transmit(buffer, 195);
                });
            }

            // We just read the correct page for this attribute. Copy it to
            // the out buffer and send it back to the client.
            State::GetAttribute{index} => {

                self.state.set(State::Idle);
                self.buffer.take().map(move |buffer| {
                    buffer[0] = ESCAPE_CHAR;
                    buffer[1] = RES_GET_ATTR;
                    let mut j = 2;
                    for i in 0..64 {
                        let b = pagebuffer.as_mut()[(((index as usize)%8)*64) + i];
                        if b == ESCAPE_CHAR {
                            // Need to escape the escape character.
                            buffer[j] = ESCAPE_CHAR;
                            j += 1;
                        }
                        buffer[j] = b;
                        j += 1;
                    }

                    self.page_buffer.replace(pagebuffer);
                    self.uart.transmit(buffer, j);
                });
            }

            // We need to update the page we just read with the new attribute,
            // and then write that all back to flash.
            State::SetAttribute{index} => {
                self.buffer.map(move |buffer| {
                    // Copy the first 64 bytes of the buffer into the correct
                    // spot in the page.
                    let start_index = ((index as usize)%8)*64;
                    for i in 0..64 {
                        pagebuffer.as_mut()[start_index + i] = buffer[i];
                    }
                    self.flash.write_page(3 + (index as usize / 8), pagebuffer);
                });
            }

            // Pass what we have read so far to the client.
            State::ReadRange{address, length, remaining_length} => {
                // Take what we need to read out of this page and send it
                // on uart. If this is the first message be sure to send the
                // header.
                self.buffer.take().map(move |buffer| {
                    let mut index = 0;
                    if length == remaining_length {
                        buffer[0] = ESCAPE_CHAR;
                        buffer[1] = RES_READ_RANGE;
                        index = 2;
                    }

                    let page_size = pagebuffer.as_mut().len();
                    // This will get us our offset into the page.
                    let page_index = address as usize % page_size;
                    // Length is either the rest of the page or how much we have left.
                    let len = cmp::min(page_size - page_index, remaining_length as usize);
                    // Make sure we don't overflow the buffer.
                    let copy_len = cmp::min(len, buffer.len()-index);

                    // Copy what we read from the page buffer to the user buffer.
                    // Keep track of how much was actually copied.
                    let mut actually_copied = 0;
                    for i in 0..copy_len {
                        // Make sure we don't overflow the buffer. We need to
                        // have at least two open bytes in the buffer
                        if index >= (buffer.len() - 1) {
                            break;
                        }

                        // Normally do the copy and check if this needs to be
                        // escaped.
                        actually_copied += 1;
                        let b = pagebuffer.as_mut()[page_index + i];
                        if b == ESCAPE_CHAR {
                            // Need to escape the escape character.
                            buffer[index] = ESCAPE_CHAR;
                            index += 1;
                        }
                        buffer[index] = b;
                        index += 1;
                    }

                    // Update our state.
                    let new_address = address as usize + actually_copied;
                    let new_remaining_length = remaining_length as usize - actually_copied;
                    self.state.set(State::ReadRange{address: new_address as u32, length, remaining_length: new_remaining_length as u16});

                    // And send the buffer to the client.
                    self.page_buffer.replace(pagebuffer);
                    self.uart.transmit(buffer, index);
                });
            }

            State::Crc{address, remaining_length, crc} => {
                let page_size = pagebuffer.as_mut().len();
                // This will get us our offset into the page.
                let page_index = address as usize % page_size;
                // Length is either the rest of the page or how much we have left.
                let len = cmp::min(page_size - page_index, remaining_length as usize);

                // Iterate all bytes in the page that are relevant to the CRC
                // and include them in the CRC calculation.
                let mut new_crc = crc;
                for i in 0..len {
                    let v1 = (new_crc ^ pagebuffer.as_mut()[page_index + i] as u32) & 0xFF;
                    let v2 = CRC32_TABLE[v1 as usize];
                    new_crc = v2 ^ (new_crc >> 8);
                }

                // Update our state.
                let new_address = address + len as u32;
                let new_remaining_length = remaining_length - len as u32;

                // Check if we are done
                if new_remaining_length == 0 {
                    // Last XOR before sending to client.
                    new_crc = new_crc ^ 0xFFFFFFFF;

                    self.state.set(State::Idle);
                    self.buffer.take().map(move |buffer| {
                        buffer[0] = ESCAPE_CHAR;
                        buffer[1] = RES_CRCIF;
                        buffer[2] = ((new_crc >> 0) & 0xFF) as u8;
                        buffer[3] = ((new_crc >> 8) & 0xFF) as u8;
                        buffer[4] = ((new_crc >> 16) & 0xFF) as u8;
                        buffer[5] = ((new_crc >> 24) & 0xFF) as u8;
                        // And send the buffer to the client.
                        self.page_buffer.replace(pagebuffer);
                        self.uart.transmit(buffer, 6);
                    });
                } else {
                    // More CRC to do!
                    self.state.set(State::Crc{address: new_address, remaining_length: new_remaining_length, crc: new_crc});
                    self.flash.read_page(new_address as usize / page_size, pagebuffer);
                }
            }

            _ => {}
        }




    }

    fn write_complete(&self, pagebuffer: &'static mut F::Page, _error: hil::flash::Error) {
        self.page_buffer.replace(pagebuffer);

        match self.state.get() {

            // Writing flash page done, send OK.
            State::WriteFlashPage => {
                self.state.set(State::Idle);
                self.buffer.take().map(move |buffer| {
                    buffer[0] = ESCAPE_CHAR;
                    buffer[1] = RES_OK;
                    // buffer[1] = 0x99;
                    self.uart.transmit(buffer, 2);
                });
            }

            // Attribute writing done, send an OK response.
            State::SetAttribute{index: _} => {
                self.state.set(State::Idle);
                self.buffer.take().map(move |buffer| {
                    buffer[0] = ESCAPE_CHAR;
                    buffer[1] = RES_OK;
                    self.uart.transmit(buffer, 2);
                });
            }

            _ => {
                self.buffer.take().map(|buffer| {
                    self.uart.receive_automatic(buffer, 250);
                });

            }
        }
    }

    fn erase_complete(&self, _error: hil::flash::Error) {
        match self.state.get() {

            // Page erased, return OK
            State::ErasePage => {
                self.state.set(State::Idle);
                self.buffer.take().map(move |buffer| {
                    buffer[0] = ESCAPE_CHAR;
                    buffer[1] = RES_OK;
                    self.uart.transmit(buffer, 2);
                });
            }

            _ => {
                self.buffer.take().map(|buffer| {
                    self.uart.receive_automatic(buffer, 250);
                });

            }
        }
    }
}
