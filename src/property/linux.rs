//! A List of Linux phandle definitions taken from the linux kernel (drivers/of/property.c)
use crate::PhandleLink;

pub const LINUX_PHANDLE_PROPERTIES_SIMPLE_LIST: &[PhandleLink] = &[
    PhandleLink {
        name: "clocks",
        size: "#clock-cells",
    },
    PhandleLink {
        name: "interconnects",
        size: "#interconnect-cells",
    },
    PhandleLink {
        name: "iommus",
        size: "#iommu-cells",
    },
    PhandleLink {
        name: "mboxes",
        size: "#mbox-cells",
    },
    PhandleLink {
        name: "io-channels",
        size: "#io-channel-cells",
    },
    PhandleLink {
        name: "io-backends",
        size: "#io-backend-cells",
    },
    PhandleLink {
        name: "dmas",
        size: "#dma-cells",
    },
    PhandleLink {
        name: "power-domains",
        size: "#power-domain-cells",
    },
    PhandleLink {
        name: "hwlocks",
        size: "#hwlock-cells",
    },
    PhandleLink {
        name: "extcon",
        size: "",
    },
    PhandleLink {
        name: "nvmem-cells",
        size: "#nvmem-cell-cells",
    },
    PhandleLink {
        name: "phys",
        size: "#phy-cells",
    },
    PhandleLink {
        name: "wakeup-parent",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-0",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-1",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-2",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-3",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-4",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-5",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-6",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-7",
        size: "",
    },
    PhandleLink {
        name: "pinctrl-8",
        size: "",
    },
    PhandleLink {
        name: "pwms",
        size: "#pwm-cells",
    },
    PhandleLink {
        name: "resets",
        size: "#reset-cells",
    },
    PhandleLink {
        name: "leds",
        size: "",
    },
    PhandleLink {
        name: "backlight",
        size: "",
    },
    PhandleLink {
        name: "panel",
        size: "",
    },
    PhandleLink {
        name: "msi-parent",
        size: "#msi-cells",
    },
    PhandleLink {
        name: "post-init-providers",
        size: "",
    },
    PhandleLink {
        name: "access-controllers",
        size: "#access-controller-cells",
    },
    PhandleLink {
        name: "pses",
        size: "#pse-cells",
    },
    PhandleLink {
        name: "power-supplies",
        size: "",
    },
];

pub const LINUX_PHANDLE_PROPERTIES_SUFFIX_LIST: &[PhandleLink] = &[
    PhandleLink {
        name: "-supply",
        size: "",
    },
    PhandleLink {
        name: "-gpio",
        size: "#gpio-cells",
    },
];
