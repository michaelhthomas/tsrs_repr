// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type InputField = { "type": "Label", text: string, } | { "type": "Input", name: string | null, placeholder: string | null, default: string | null, } & ({ "input_type": "Text" } | { "input_type": "Expression" } | { "input_type": "Number", min: number | null, max: number | null, } | { "input_type": "Dropdown", options: Array<[string, string]>, }) };
