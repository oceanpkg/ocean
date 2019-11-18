//! Known licensing options.

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

/// Counts the number of input tokens.
macro_rules! count {
    () => (0usize);
    ($x:tt $($xs:tt)*) => (1usize + count!($($xs)*));
}

macro_rules! def_license {
    ($($(#[$m:meta])+ $l:ident = $s:literal,)+) => {
        /// A known licensing option.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum License {
            $($(#[$m])+ $l,)+
        }

        static BY_LICENSE: [&str; count!($($l)+)] = [$($s,)+];
    };
}

def_license! {
    /// Zero-Clause BSD / Free Public License 1.0.0 (0BSD OR BSD-0-Clause).
    Bsd0Clause = "BSD-0-Clause",
    /// 2-clause BSD License (BSD-2-Clause).
    Bsd2Clause = "BSD-2-Clause",
    /// BSD+Patent (BSD-2-Clause-Patent).
    Bsd2ClausePatent = "BSD-2-Clause-Patent",
    /// 3-clause BSD License (BSD-3-Clause).
    Bsd3Clause = "BSD-3-Clause",
    /// Lawrence Berkeley National Labs BSD Variant License (BSD-3-Clause-LBNL).
    Bsd3ClauseLbnl = "BSD-3-Clause-LBNL",
    /// BSD License (BSD).
    Bsd = "BSD",
    /// Academic Free License 3.0 (AFL-3.0).
    Afl3 = "AFL-3.0",
    /// Adaptive Public License (APL-1.0).
    Apl1 = "APL-1.0",
    /// Apache Software License 1.1 (Apache-1.1).
    Apache1_1 = "Apache-1.1",
    /// Apache License 2.0 (Apache-2.0).
    Apache2 = "Apache-2.0",
    /// Apple Public Source License (APSL-2.0).
    Apsl2 = "APSL-2.0",
    /// Artistic license 1.0 (Artistic-1.0).
    Artistic1 = "Artistic-1.0",
    /// Artistic License 2.0 (Artistic-2.0).
    Artistic2 = "Artistic-2.0",
    /// Attribution Assurance License (AAL).
    Aal = "AAL",
    /// Boost Software License (BSL-1.0).
    Bsl1 = "BSL-1.0",
    /// CeCILL License 2.1 (CECILL-2.1).
    Cecill2_1 = "CECILL-2.1",
    /// Common Development and Distribution License 1.0 (CDDL-1.0).
    Cddl1 = "CDDL-1.0",
    /// Common Public Attribution License 1.0 (CPAL-1.0).
    Cpal1 = "CPAL-1.0",
    /// Common Public License 1.0 (CPL-1.0).
    Cpl1 = "CPL-1.0",
    /// Computer Associates Trusted Open Source License 1.1 (CATOSL-1.1).
    Catosl1_1 = "CATOSL-1.1",
    /// CUA Office Public License Version 1.0 (CUA-OPL-1.0).
    CuaOpl1 = "CUA-OPL-1.0",
    /// Eclipse Public License 1.0 (EPL-1.0).
    Epl1 = "EPL-1.0",
    /// Eclipse Public License 2.0 (EPL-2.0).
    Epl2 = "EPL-2.0",
    /// eCos License version 2.0 (eCos-2.0).
    ECos2 = "eCos-2.0",
    /// Educational Community License, Version 1.0 (ECL-1.0).
    Ecl1 = "ECL-1.0",
    /// Educational Community License, Version 2.0 (ECL-2.0).
    Ecl2 = "ECL-2.0",
    /// Eiffel Forum License V1.0 (EFL-1.0).
    Efl1 = "EFL-1.0",
    /// Eiffel Forum License V2.0 (EFL-2.0).
    Efl2 = "EFL-2.0",
    /// Entessa Public License (Entessa).
    Entessa = "Entessa",
    /// EU DataGrid Software License (EUDatagrid).
    EUDatagrid = "EUDatagrid",
    /// European Union Public License 1.2 (EUPL-1.2).
    Eupl1_2 = "EUPL-1.2",
    /// Fair License (Fair).
    Fair = "Fair",
    /// Frameworx License (Frameworx-1.0).
    Frameworx1 = "Frameworx-1.0",
    /// GNU Affero General Public License version 3 (AGPL-3.0).
    AGPL3 = "AGPL-3.0",
    /// GNU General Public License version 2 (GPL-2.0).
    Gpl2 = "GPL-2.0",
    /// GNU General Public License version 3 (GPL-3.0).
    Gpl3 = "GPL-3.0",
    /// GNU Lesser General Public License version 2.1 (LGPL-2.1).
    Lgpl2_1 = "LGPL-2.1",
    /// GNU Lesser General Public License version 3 (LGPL-3.0).
    Lgpl3 = "LGPL-3.0",
    /// Historical Permission Notice and Disclaimer (HPND).
    Hpnd = "HPND",
    /// IBM Public License 1.0 (IPL-1.0).
    Ipl1 = "IPL-1.0",
    /// Intel Open Source License (Intel).
    Intel = "Intel",
    /// IPA Font License (IPA).
    Ipa = "IPA",
    /// ISC License (ISC).
    Isc = "ISC",
    /// Jabber Open Source License (Jabber).
    Jabber = "Jabber",
    /// LaTeX Project Public License 1.3c (LPPL-1.3c).
    LPPL1_3c = "LPPL-1.3c",
    /// Licence Libre du Québec – Permissive 1.1 (LiLiQ-P-1.1).
    LiliQP1_1 = "LiLiQ-P-1.1",
    /// Licence Libre du Québec – Réciprocité 1.1 (LiLiQ-R-1.1).
    LiliQR1_1 = "LiLiQ-R-1.1",
    /// Licence Libre du Québec – Réciprocité forte 1.1 (LiLiQ-R+-1.1).
    LiliQRF1_1 = "LiLiQ-R+-1.1",
    /// Lucent Public License, "Plan9", version 1.0 (LPL-1.0).
    Lpl1 = "LPL-1.0",
    /// Lucent Public License Version 1.02 (LPL-1.02).
    Lpl1_02 = "LPL-1.02",
    /// Microsoft Public License (MS-PL).
    MsPl = "MS-PL",
    /// Microsoft Reciprocal License (MS-RL).
    MsRl = "MS-RL",
    /// MirOS Licence (MirOS).
    MirOs = "MirOS",
    /// MIT License (MIT).
    Mit = "MIT",
    /// MITRE Collaborative Virtual Workspace License (CVW).
    Cvw = "CVW",
    /// Motosoto License (Motosoto).
    Motosoto = "Motosoto",
    /// Mozilla Public License 1.0 (MPL-1.0).
    Mpl1 = "MPL-1.0",
    /// Mozilla Public License 1.1 (MPL-1.1).
    Mpl1_1 = "MPL-1.1",
    /// Mozilla Public License 2.0 (MPL-2.0).
    Mpl2 = "MPL-2.0",
    /// Multics License (Multics).
    Multics = "Multics",
    /// NASA Open Source Agreement 1.3 (NASA-1.3).
    Nasa1_3 = "NASA-1.3",
    /// Naumen Public License (Naumen).
    Naumen = "Naumen",
    /// Nethack General Public License (NGPL).
    Ngpl = "NGPL",
    /// Nokia Open Source License (Nokia).
    Nokia = "Nokia",
    /// Non-Profit Open Software License 3.0 (NPOSL-3.0).
    Nposl3 = "NPOSL-3.0",
    /// NTP License (NTP).
    Ntp = "NTP",
    /// OCLC Research Public License 2.0 (OCLC-2.0).
    Oclc2 = "OCLC-2.0",
    /// Open Group Test Suite License (OGTSL).
    Ogtsl = "OGTSL",
    /// Open Software License 1.0 (OSL-1.0).
    Osl1 = "OSL-1.0",
    /// Open Software License 2.1 (OSL-2.1).
    Osl2_1 = "OSL-2.1",
    /// Open Software License 3.0 (OSL-3.0).
    Osl3 = "OSL-3.0",
    /// OSET Public License version 2.1 (OSET-2.1).
    Oset2_1 = "OSET-2.1",
    /// PHP License 3.0 (PHP-3.0).
    Php3 = "PHP-3.0",
    /// The PostgreSQL License (PostgreSQL).
    PostgreSql = "PostgreSQL",
    /// Python License (Python-1.0).
    Python1 = "Python-1.0",
    /// Python License (Python-2.0).
    Python2 = "Python-2.0",
    /// CNRI Python license (CNRI-Python).
    CnriPython = "CNRI-Python",
    /// Q Public License (QPL-1.0).
    Qpl1 = "QPL-1.0",
    /// RealNetworks Public Source License V1.0 (RPSL-1.0).
    Rpsl1 = "RPSL-1.0",
    /// Reciprocal Public License, version 1.1 (RPL-1.1).
    Rpl1_1 = "RPL-1.1",
    /// Reciprocal Public License 1.5 (RPL-1.5).
    Rpl1_5 = "RPL-1.5",
    /// Ricoh Source Code Public License (RSCPL).
    Rscpl = "RSCPL",
    /// SIL Open Font License 1.1 (OFL-1.1).
    Ofl1_1 = "OFL-1.1",
    /// Simple Public License 2.0 (SimPL-2.0).
    SimPl2_0 = "SimPL-2.0",
    /// Sleepycat License (Sleepycat).
    Sleepycat = "Sleepycat",
    /// Sun Industry Standards Source License (SISSL).
    Sissl = "SISSL",
    /// Sun Public License 1.0 (SPL-1.0).
    Spl1 = "SPL-1.0",
    /// Sybase Open Watcom Public License 1.0 (Watcom-1.0).
    Watcom1 = "Watcom-1.0",
    /// Universal Permissive License (UPL).
    Upl = "UPL",
    /// University of Illinois/NCSA Open Source License (NCSA).
    Ncsa = "NCSA",
    /// Upstream Compatibility License v1.0 (UCL-1.0).
    Ucl1 = "UCL-1.0",
    /// Vovida Software License v. 1.0 (VSL-1.0).
    Vsl1 = "VSL-1.0",
    /// W3C License (W3C).
    W3C = "W3C",
    /// wxWindows Library License (WXwindows).
    WxWindows = "WXwindows",
    /// X.Net License (Xnet).
    Xnet = "Xnet",
    /// Zope Public License 2.0 (ZPL-2.0).
    Zpl2 = "ZPL-2.0",
    /// zlib/libpng license (Zlib).
    Zlib = "Zlib",
}

static BY_NAME: phf::Map<&[u8], License> = {
    use self::License::*;
    phf_map! {
        b"BSD-0-Clause"        => Bsd0Clause,
        b"0BSD"                => Bsd0Clause, // Alias
        b"BSD-2-Clause"        => Bsd2Clause,
        b"2BSD"                => Bsd2Clause, // Alias
        b"BSD-2-Clause-Patent" => Bsd2ClausePatent,
        b"BSD-3-Clause"        => Bsd3Clause,
        b"3BSD"                => Bsd3Clause, // Alias
        b"BSD-3-Clause-LBNL"   => Bsd3ClauseLbnl,
        b"BSD"                 => Bsd,
        b"AFL-3.0"             => Afl3,
        b"AFL-3"               => Afl3, // Alias
        b"APL-1.0"             => Apl1,
        b"APL-1"               => Apl1, // Alias
        b"Apache-1.1"          => Apache1_1,
        b"Apache-2.0"          => Apache2,
        b"Apache-2"            => Apache2, // Alias
        b"APSL-2.0"            => Apsl2,
        b"APSL-2"              => Apsl2, // Alias
        b"Artistic-1.0"        => Artistic1,
        b"Artistic-1"          => Artistic1, // Alias
        b"Artistic-2.0"        => Artistic2,
        b"Artistic-2"          => Artistic2, // Alias
        b"AAL"                 => Aal,
        b"BSL-1.0"             => Bsl1,
        b"BSL-1"               => Bsl1, // Alias
        b"CECILL-2.1"          => Cecill2_1,
        b"CDDL-1.0"            => Cddl1,
        b"CDDL-1"              => Cddl1, // Alias
        b"CPAL-1.0"            => Cpal1,
        b"CPAL-1"              => Cpal1, // Alias
        b"CPL-1.0"             => Cpl1,
        b"CPL-1"               => Cpl1, // Alias
        b"CATOSL-1.1"          => Catosl1_1,
        b"CUA-OPL-1.0"         => CuaOpl1,
        b"CUA-OPL-1"           => CuaOpl1, // Alias
        b"EPL-1.0"             => Epl1,
        b"EPL-1"               => Epl1, // Alias
        b"EPL-2.0"             => Epl2,
        b"EPL-2"               => Epl2, // Alias
        b"eCos-2.0"            => ECos2,
        b"eCos-2"              => ECos2, // Alias
        b"ECL-1.0"             => Ecl1,
        b"ECL-1"               => Ecl1, // Alias
        b"ECL-2.0"             => Ecl2,
        b"ECL-2"               => Ecl2, // Alias
        b"EFL-1.0"             => Efl1,
        b"EFL-1"               => Efl1, // Alias
        b"EFL-2.0"             => Efl2,
        b"EFL-2"               => Efl2, // Alias
        b"Entessa"             => Entessa,
        b"EUDatagrid"          => EUDatagrid,
        b"EUPL-1.2"            => Eupl1_2,
        b"Fair"                => Fair,
        b"Frameworx-1.0"       => Frameworx1,
        b"Frameworx-1"         => Frameworx1, // Alias
        b"AGPL-3.0"            => AGPL3,
        b"AGPL-3"              => AGPL3, // Alias
        b"GPL-2.0"             => Gpl2,
        b"GPL-2"               => Gpl2, // Alias
        b"GPL-3.0"             => Gpl3,
        b"GPL-3"               => Gpl3, // Alias
        b"LGPL-2.1"            => Lgpl2_1,
        b"LGPL-3.0"            => Lgpl3,
        b"LGPL-3"              => Lgpl3, // Alias
        b"HPND"                => Hpnd,
        b"IPL-1.0"             => Ipl1,
        b"IPL-1"               => Ipl1, // Alias
        b"Intel"               => Intel,
        b"IPA"                 => Ipa,
        b"ISC"                 => Isc,
        b"Jabber"              => Jabber,
        b"LPPL-1.3c"           => LPPL1_3c,
        b"LiLiQ-P-1.1"         => LiliQP1_1,
        b"LiLiQ-R-1.1"         => LiliQR1_1,
        b"LiLiQ-R+-1.1"        => LiliQRF1_1,
        b"LPL-1.0"             => Lpl1,
        b"LPL-1"               => Lpl1, // Alias
        b"LPL-1.02"            => Lpl1_02,
        b"LPL-12"              => Lpl1_02, // Alias
        b"MS-PL"               => MsPl,
        b"MS-RL"               => MsRl,
        b"MirOS"               => MirOs,
        b"MIT"                 => Mit,
        b"CVW"                 => Cvw,
        b"Motosoto"            => Motosoto,
        b"MPL-1.0"             => Mpl1,
        b"MPL-1"               => Mpl1, // Alias
        b"MPL-1.1"             => Mpl1_1,
        b"MPL-2.0"             => Mpl2,
        b"MPL-2"               => Mpl2, // Alias
        b"Multics"             => Multics,
        b"NASA-1.3"            => Nasa1_3,
        b"Naumen"              => Naumen,
        b"NGPL"                => Ngpl,
        b"Nokia"               => Nokia,
        b"NPOSL-3.0"           => Nposl3,
        b"NPOSL-3"             => Nposl3, // Alias
        b"NTP"                 => Ntp,
        b"OCLC-2.0"            => Oclc2,
        b"OCLC-2"              => Oclc2, // Alias
        b"OGTSL"               => Ogtsl,
        b"OSL-1.0"             => Osl1,
        b"OSL-1"               => Osl1, // Alias
        b"OSL-2.1"             => Osl2_1,
        b"OSL-3.0"             => Osl3,
        b"OSL-3"               => Osl3, // Alias
        b"OSET-2.1"            => Oset2_1,
        b"PHP-3.0"             => Php3,
        b"PHP-3"               => Php3, // Alias
        b"PostgreSQL"          => PostgreSql,
        b"Python-1.0"          => Python1,
        b"Python-1"            => Python1, // Alias
        b"Python-2.0"          => Python2,
        b"Python-2"            => Python2, // Alias
        b"CNRI-Python"         => CnriPython,
        b"QPL-1.0"             => Qpl1,
        b"QPL-1"               => Qpl1, // Alias
        b"RPSL-1.0"            => Rpsl1,
        b"RPSL-1"              => Rpsl1, // Alias
        b"RPL-1.1"             => Rpl1_1,
        b"RPL-1.5"             => Rpl1_5,
        b"RSCPL"               => Rscpl,
        b"OFL-1.1"             => Ofl1_1,
        b"SimPL-2.0"           => SimPl2_0,
        b"SimPL-2"             => SimPl2_0, // Alias
        b"Sleepycat"           => Sleepycat,
        b"SISSL"               => Sissl,
        b"SPL-1.0"             => Spl1,
        b"SPL-1"               => Spl1, // Alias
        b"Watcom-1.0"          => Watcom1,
        b"Watcom-1"            => Watcom1, // Alias
        b"UPL"                 => Upl,
        b"NCSA"                => Ncsa,
        b"UCL-1.0"             => Ucl1,
        b"UCL-1"               => Ucl1, // Alias
        b"VSL-1.0"             => Vsl1,
        b"VSL-1"               => Vsl1, // Alias
        b"W3C"                 => W3C,
        b"WXwindows"           => WxWindows,
        b"Xnet"                => Xnet,
        b"ZPL-2.0"             => Zpl2,
        b"ZPL-2"               => Zpl2, // Alias
        b"Zlib"                => Zlib,
    }
};

/// An error returned when a license name is unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError(());

impl TryFrom<&[u8]> for License {
    type Error = ParseError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        BY_NAME.get(bytes)
            .map(|&license| license)
            .ok_or(ParseError(()))
    }
}

impl TryFrom<&str> for License {
    type Error = ParseError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.as_bytes())
    }
}

impl fmt::Display for License {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl License {
    /// Attempts to parse `input` and returns a
    /// [`ParseError`](struct.ParseError.html) on error.
    #[inline]
    pub fn parse<I>(input: I) -> Result<Self, ParseError>
        where I: TryInto<Self, Error = ParseError>
    {
        input.try_into()
    }

    /// Returns the string identifier for this license.
    #[inline]
    pub fn as_str(&self) -> &'static str {
        BY_LICENSE[*self as usize]
    }
}
