//! render.rs
//! 
//! utility functions for printing/rendering data

use fugue_ir::{
    VarnodeData, Translator,
    disassembly::lift::PCodeData
};


// ////////////////////////////////////////////////////////////////////////////
// FUGUE RELATED UTILITES

/// a varnode formatter that spits out a smaller string
pub fn fmt_vnd(vnd: &VarnodeData, t: &Translator, hex_const: Option<bool>) -> String {
    let hex_const = hex_const.unwrap_or(false);
    let space = t.manager().unchecked_space_by_id(vnd.space());
    if space.is_register() {
        let name = t.registers()
            .get(vnd.offset(), vnd.size())
            .unwrap();
        return format!("{}", name)
    } else if space.is_constant() {
        if hex_const {
            return format!("{:#x}", vnd.offset())
        }
        return format!("{}", vnd.offset())
    } else {
        return format!("{}[{:#x}; {}]", space.name(), vnd.offset(), vnd.size())
    }
}

/// a pcodedata formatter that spits out a more intuitive format:
/// [<Out> = ]<Op>(<in0>[, <in1>[, <in2>]])
pub fn fmt_pcodeop(pcodeop: &PCodeData, t: &Translator, hex_const: Option<bool>) -> String {
    let mut result = String::new();
    // [<Out> = ]
    if let Some(output) = pcodeop.output {
        result.push_str(&format!("{} = ", fmt_vnd(&output, t, hex_const)));
    }
    // <Op>
    result.push_str(&format!("{:?}(", pcodeop.opcode));
    // (<in0>[, <in1>[, <in2>]])
    for (i, input) in pcodeop.inputs.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(&format!("{}", fmt_vnd(input, t, hex_const)));
    }
    result.push(')');
    result
}

