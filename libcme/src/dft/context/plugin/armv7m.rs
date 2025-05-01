//! armv7m.rs
//! 
//! armv7m plugin

use super::*;

#[derive(Debug, Clone, Default)]
pub struct Armv7m;


impl Armv7m {
    /// perform tag propagation for exception entry
    /// primarily associated with pushing stack
    /// returns the tag value of the exception vector read from the vector table
    /// see B1.5.8
    fn exception_entry(
        &mut self,
        shadow: &mut ShadowState,
        ctx_switch: &backend::ThreadSwitch,
        nofpext: bool,
    ) -> Result<Tag, Error> {
        // to support floating point, need to do extended stack
        assert_eq!(nofpext, true, "floating point not supported");
        // let framesize = 0x20;
        let frame_address = ctx_switch.new_frame_address;
        assert!(ctx_switch.return_address.is_some(),
            "exception entry must have associated return address");
        let return_address = ctx_switch.return_address.unwrap();

        // if the return address exists, the vtor must as well
        let vtor = ctx_switch.vtor.unwrap();
        
        // push register tags
        let lang = shadow.lang.clone();
        let t = lang.translator();
        let lr_vnd = t.register_by_name("lr").unwrap();
        let push_regs = ["r0", "r1", "r2", "r3", "r12", "lr", ""].into_iter()
            .map(|reg_str| {
                t.register_by_name(reg_str).unwrap()
            });
        for (i, reg) in push_regs.enumerate() {
            let tag = shadow.read_tag(&reg)?;
            shadow.write_mem_tags(&(frame_address + i * 4), 4, tag)?;
        }

        // push return address tag
        let return_address_tag = shadow.read_mem_tags(return_address, 4)?;
        shadow.write_mem_tags(&(frame_address + 0x18u64), 4, return_address_tag)?;
        // push xpsr tag (always clean)
        shadow.write_mem_tags(&(frame_address + 0x1Cu64), 4, Tag::from(tag::ACCESSED))?;

        shadow.write_tag(&lr_vnd, &Tag::from(tag::ACCESSED))?;

        // get target address tag from exception typ
        assert_ne!(ctx_switch.typ, 0, "exception entry cannot have exception number 0");
        
        Ok(shadow.read_mem_tags(&(vtor + ctx_switch.typ * 4), 4)?)
    }

    /// perform tag propagation for exception return
    /// pops stack tag values to registers
    /// returns the tag value of the stack location where the 
    /// return address was stored
    /// see B1.5.8
    fn exception_return(
        &mut self,
        shadow: &mut ShadowState,
        ctx_switch: &backend::ThreadSwitch,
        nofpext: bool,
    ) -> Result<Tag, Error> {
        assert_eq!(nofpext, true, "floating point not supported");
        // let framesize = 0x20
        // might want to check the frame address for taint, but incorporate that later...
        let frame_address = ctx_switch.old_frame_address;
        assert!(ctx_switch.return_address.is_none(),
            "exception return cannot have a return address");
        let lang = shadow.lang.clone();
        let t = lang.translator();
        let pop_regs = ["r0", "r1", "r2", "r3", "r12", "lr", ""].into_iter()
            .map(|reg_str| {
                t.register_by_name(reg_str).unwrap()
            });

        for (i, reg) in pop_regs.enumerate() {
            let tag = shadow
                .read_mem_tags(&(frame_address + i as u32 * 4), 4)?;
            shadow.write_tag(&reg, &tag)?;
        }
        let target_address_tag = shadow
            .read_mem_tags(&(frame_address + 0x18u64), 4)?;
        
        // maybe want to check psr tag at some point, it's probably bad if it gets tainted
        let _psr_tag = shadow
            .read_mem_tags(&(frame_address + 0x1Cu64), 4)?;

        Ok(target_address_tag)
    }
}

impl ArchPlugin for Armv7m {
    fn maybe_thread_switch(
        &mut self,
        shadow: &mut ShadowState,
        ctx_switch: &backend::ThreadSwitch,
    ) -> Result<Tag, Error> {
        if ctx_switch.return_address.is_some() {
            // switch to handler
            self.exception_entry(shadow, ctx_switch, true)
        } else {
            // return from handler
            self.exception_return(shadow, ctx_switch, true)
        }
    }
}