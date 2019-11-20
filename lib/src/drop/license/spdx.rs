//! A commonly found license listed [here](https://spdx.org/licenses).

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};

macro_rules! spdx_license {
    ($($l:ident = $id:literal, $name:literal;)+) => {
        /// A commonly found license listed [here](https://spdx.org/licenses).
        ///
        /// This list is based on version 3.7 (2019-10-22).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        // TODO: Add `#[non_exhaustive]` when stable
        pub enum SpdxLicense {
            $(
                #[doc = $name]
                #[doc = "— `"]
                #[doc = $id]
                #[doc = "`."]
                $l,
            )+
        }

        impl SpdxLicense {
            const _COUNT: usize = count!($($l)+);
        }

        static LICENSE_BY_ID: phf::Map<&str, SpdxLicense> = phf::phf_map! {
            $($id => SpdxLicense::$l,)+
        };

        const ID_BY_LICENSE:   Map<&str> = [$($id,)+];
        const NAME_BY_LICENSE: Map<&str> = [$($name,)+];
    };
}

spdx_license! {
    Bsd0 = "BSD 0", "BSD Zero Clause License";
    Aal = "AAL", "Attribution Assurance License";
    Abstyles = "Abstyles", "Abstyles License";
    Adobe2006 = "Adobe-2006", "Adobe Systems Incorporated Source Code License Agreement";
    AdobeGlyph = "Adobe-Glyph", "Adobe Glyph List License";
    Adsl = "ADSL", "Amazon Digital Services License";
    Afl1_1 = "AFL-1.1", "Academic Free License v1.1";
    Afl1_2 = "AFL-1.2", "Academic Free License v1.2";
    Afl2 = "AFL-2.0", "Academic Free License v2.0";
    Afl2_1 = "AFL-2.1", "Academic Free License v2.1";
    Afl3 = "AFL-3.0", "Academic Free License v3.0";
    Afmparse = "Afmparse", "Afmparse License";
    Agpl1Only = "AGPL-1.0-only", "Affero General Public License v1.0 only";
    Agpl1OrLater = "AGPL-1.0-or-later", "Affero General Public License v1.0 or later";
    Agpl3Only = "AGPL-3.0-only", "GNU Affero General Public License v3.0 only";
    Agpl3OrLater = "AGPL-3.0-or-later", "GNU Affero General Public License v3.0 or later";
    Aladdin = "Aladdin", "Aladdin Free Public License";
    Amdplpa = "AMDPLPA", "AMD's plpa_map.c License";
    Aml = "AML", "Apple MIT License";
    Ampas = "AMPAS", "Academy of Motion Picture Arts and Sciences BSD";
    AntlrPd = "ANTLR-PD", "ANTLR Software Rights Notice";
    Apache1 = "Apache-1.0", "Apache License 1.0";
    Apache1_1 = "Apache-1.1", "Apache License 1.1";
    Apache2 = "Apache-2.0", "Apache License 2.0";
    Apafml = "APAFML", "Adobe Postscript AFM License";
    Apl1 = "APL-1.0", "Adaptive Public License 1.0";
    Apsl1 = "APSL-1.0", "Apple Public Source License 1.0";
    Apsl1_1 = "APSL-1.1", "Apple Public Source License 1.1";
    Apsl1_2 = "APSL-1.2", "Apple Public Source License 1.2";
    Apsl2 = "APSL-2.0", "Apple Public Source License 2.0";
    Artistic1 = "Artistic-1.0", "Artistic License 1.0";
    Artistic1Cl8 = "Artistic-1.0-cl8", "Artistic License 1.0 w/clause 8";
    Artistic1Perl = "Artistic-1.0-Perl", "Artistic License 1.0 (Perl)";
    Artistic2 = "Artistic-2.0", "Artistic License 2.0";
    Bahyph = "Bahyph", "Bahyph License";
    Barr = "Barr", "Barr License";
    Beerware = "Beerware", "Beerware License";
    BitTorrent1 = "BitTorrent-1.0", "BitTorrent Open Source License v1.0";
    BitTorrent1_1 = "BitTorrent-1.1", "BitTorrent Open Source License v1.1";
    Blessing = "blessing", "SQLite Blessing";
    BlueOak1 = "BlueOak-1.0.0", "Blue Oak Model License 1.0.0";
    Borceux = "Borceux", "Borceux license";
    Bsd1Clause = "BSD-1-Clause", "BSD 1-Clause License";
    Bsd2Clause = "BSD-2-Clause", "BSD 2-Clause \"Simplified\" License";
    Bsd2ClauseFreeBsd = "BSD-2-Clause-FreeBSD", "BSD 2-Clause FreeBSD License";
    Bsd2ClauseNetBsd = "BSD-2-Clause-NetBSD", "BSD 2-Clause NetBSD License";
    Bsd2ClausePatent = "BSD-2-Clause-Patent", "BSD-2-Clause Plus Patent License";
    Bsd3Clause = "BSD-3-Clause", "BSD 3-Clause \"New\" or \"Revised\" License";
    Bsd3ClauseAttribution = "BSD-3-Clause-Attribution", "BSD with attribution";
    Bsd3ClauseClear = "BSD-3-Clause-Clear", "BSD 3-Clause Clear License";
    Bsd3ClauseLbnl = "BSD-3-Clause-LBNL", "Lawrence Berkeley National Labs BSD variant license";
    Bsd3ClauseNoNuclearLicense = "BSD-3-Clause-No-Nuclear-License", "BSD 3-Clause No Nuclear License";
    Bsd3ClauseNoNuclearLicense2014 = "BSD-3-Clause-No-Nuclear-License-2014", "BSD 3-Clause No Nuclear License 2014";
    Bsd3ClauseNoNuclearWarranty = "BSD-3-Clause-No-Nuclear-Warranty", "BSD 3-Clause No Nuclear Warranty";
    Bsd3ClauseOpenMpi = "BSD-3-Clause-Open-MPI", "BSD 3-Clause Open MPI variant";
    Bsd4Clause = "BSD-4-Clause", "BSD 4-Clause \"Original\" or \"Old\" License";
    Bsd4ClauseUc = "BSD-4-Clause-UC", "BSD-4-Clause (University of California-Specific)";
    BsdProtection = "BSD-Protection", "BSD Protection License";
    BsdSourceCode = "BSD-Source-Code", "BSD Source Code Attribution";
    Bsl1 = "BSL-1.0", "Boost Software License 1.0";
    Bzip21_0_5 = "bzip2-1.0.5", "bzip2 and libbzip2 License v1.0.5";
    Bzip21_0_6 = "bzip2-1.0.6", "bzip2 and libbzip2 License v1.0.6";
    Caldera = "Caldera", "Caldera License";
    Catosl1_1 = "CATOSL-1.1", "Computer Associates Trusted Open Source License 1.1";
    CcBy1 = "CC-BY-1.0", "Creative Commons Attribution 1.0 Generic";
    CcBy2 = "CC-BY-2.0", "Creative Commons Attribution 2.0 Generic";
    CcBy2_5 = "CC-BY-2.5", "Creative Commons Attribution 2.5 Generic";
    CcBy3 = "CC-BY-3.0", "Creative Commons Attribution 3.0 Unported";
    CcBy4 = "CC-BY-4.0", "Creative Commons Attribution 4.0 International";
    CcByNc1 = "CC-BY-NC-1.0", "Creative Commons Attribution Non Commercial 1.0 Generic";
    CcByNc2 = "CC-BY-NC-2.0", "Creative Commons Attribution Non Commercial 2.0 Generic";
    CcByNc2_5 = "CC-BY-NC-2.5", "Creative Commons Attribution Non Commercial 2.5 Generic";
    CcByNc3 = "CC-BY-NC-3.0", "Creative Commons Attribution Non Commercial 3.0 Unported";
    CcByNc4 = "CC-BY-NC-4.0", "Creative Commons Attribution Non Commercial 4.0 International";
    CcByNcNd1 = "CC-BY-NC-ND-1.0", "Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic";
    CcByNcNd2 = "CC-BY-NC-ND-2.0", "Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic";
    CcByNcNd2_5 = "CC-BY-NC-ND-2.5", "Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic";
    CcByNcNd3 = "CC-BY-NC-ND-3.0", "Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported";
    CcByNcNd4 = "CC-BY-NC-ND-4.0", "Creative Commons Attribution Non Commercial No Derivatives 4.0 International";
    CcByNcSa1 = "CC-BY-NC-SA-1.0", "Creative Commons Attribution Non Commercial Share Alike 1.0 Generic";
    CcByNcSa2 = "CC-BY-NC-SA-2.0", "Creative Commons Attribution Non Commercial Share Alike 2.0 Generic";
    CcByNcSa2_5 = "CC-BY-NC-SA-2.5", "Creative Commons Attribution Non Commercial Share Alike 2.5 Generic";
    CcByNcSa3 = "CC-BY-NC-SA-3.0", "Creative Commons Attribution Non Commercial Share Alike 3.0 Unported";
    CcByNcSa4 = "CC-BY-NC-SA-4.0", "Creative Commons Attribution Non Commercial Share Alike 4.0 International";
    CcByNd1 = "CC-BY-ND-1.0", "Creative Commons Attribution No Derivatives 1.0 Generic";
    CcByNd2 = "CC-BY-ND-2.0", "Creative Commons Attribution No Derivatives 2.0 Generic";
    CcByNd2_5 = "CC-BY-ND-2.5", "Creative Commons Attribution No Derivatives 2.5 Generic";
    CcByNd3 = "CC-BY-ND-3.0", "Creative Commons Attribution No Derivatives 3.0 Unported";
    CcByNd4 = "CC-BY-ND-4.0", "Creative Commons Attribution No Derivatives 4.0 International";
    CcBySa1 = "CC-BY-SA-1.0", "Creative Commons Attribution Share Alike 1.0 Generic";
    CcBySa2 = "CC-BY-SA-2.0", "Creative Commons Attribution Share Alike 2.0 Generic";
    CcBySa2_5 = "CC-BY-SA-2.5", "Creative Commons Attribution Share Alike 2.5 Generic";
    CcBySa3 = "CC-BY-SA-3.0", "Creative Commons Attribution Share Alike 3.0 Unported";
    CcBySa4 = "CC-BY-SA-4.0", "Creative Commons Attribution Share Alike 4.0 International";
    CcPddc = "CC-PDDC", "Creative Commons Public Domain Dedication and Certification";
    CC01 = "CC0-1.0", "Creative Commons Zero v1.0 Universal";
    Cddl1 = "CDDL-1.0", "Common Development and Distribution License 1.0";
    Cddl1_1 = "CDDL-1.1", "Common Development and Distribution License 1.1";
    CdlaPermissive1 = "CDLA-Permissive-1.0", "Community Data License Agreement Permissive 1.0";
    CdlaSharing1 = "CDLA-Sharing-1.0", "Community Data License Agreement Sharing 1.0";
    Cecill1 = "CECILL-1.0", "CeCILL Free Software License Agreement v1.0";
    Cecill1_1 = "CECILL-1.1", "CeCILL Free Software License Agreement v1.1";
    Cecill2 = "CECILL-2.0", "CeCILL Free Software License Agreement v2.0";
    Cecill2_1 = "CECILL-2.1", "CeCILL Free Software License Agreement v2.1";
    CecillB = "CECILL-B", "CeCILL-B Free Software License Agreement";
    CecillC = "CECILL-C", "CeCILL-C Free Software License Agreement";
    CernOhl1_1 = "CERN-OHL-1.1", "CERN Open Hardware Licence v1.1";
    CernOhl1_2 = "CERN-OHL-1.2", "CERN Open Hardware Licence v1.2";
    ClArtistic = "ClArtistic", "Clarified Artistic License";
    CnriJython = "CNRI-Jython", "CNRI Jython License";
    CnriPython = "CNRI-Python", "CNRI Python License";
    CnriPythonGplCompatible = "CNRI-Python-GPL-Compatible", "CNRI Python Open Source GPL Compatible License Agreement";
    Condor1_1 = "Condor-1.1", "Condor Public License v1.1";
    CopyleftNext0_3 = "copyleft-next-0.3.0", "copyleft-next 0.3.0";
    CopyleftNext0_3_1 = "copyleft-next-0.3.1", "copyleft-next 0.3.1";
    Cpal1 = "CPAL-1.0", "Common Public Attribution License 1.0";
    Cpl1 = "CPL-1.0", "Common Public License 1.0";
    Cpol1_02 = "CPOL-1.02", "Code Project Open License 1.02";
    Crossword = "Crossword", "Crossword License";
    CrystalStacker = "CrystalStacker", "CrystalStacker License";
    CuaOpl1 = "CUA-OPL-1.0", "CUA Office Public License v1.0";
    Cube = "Cube", "Cube License";
    Curl = "curl", "curl License";
    DFsl1 = "D-FSL-1.0", "Deutsche Freie Software Lizenz";
    Diffmark = "diffmark", "diffmark license";
    Doc = "DOC", "DOC License";
    Dotseqn = "Dotseqn", "Dotseqn License";
    Dsdp = "DSDP", "DSDP License";
    Dvipdfm = "dvipdfm", "dvipdfm License";
    Ecl1 = "ECL-1.0", "Educational Community License v1.0";
    Ecl2 = "ECL-2.0", "Educational Community License v2.0";
    Efl1 = "EFL-1.0", "Eiffel Forum License v1.0";
    Efl2 = "EFL-2.0", "Eiffel Forum License v2.0";
    EGenix = "eGenix", "eGenix.com Public License 1.1.0";
    Entessa = "Entessa", "Entessa Public License v1.0";
    Epl1 = "EPL-1.0", "Eclipse Public License 1.0";
    Epl2 = "EPL-2.0", "Eclipse Public License 2.0";
    ErlPl1_1 = "ErlPL-1.1", "Erlang Public License v1.1";
    Etalab2 = "etalab-2.0", "Etalab Open License 2.0";
    EUDatagrid = "EUDatagrid", "EU DataGrid Software License";
    Eupl1 = "EUPL-1.0", "European Union Public License 1.0";
    Eupl1_1 = "EUPL-1.1", "European Union Public License 1.1";
    Eupl1_2 = "EUPL-1.2", "European Union Public License 1.2";
    Eurosym = "Eurosym", "Eurosym License";
    Fair = "Fair", "Fair License";
    Frameworx1 = "Frameworx-1.0", "Frameworx Open License 1.0";
    FreeImage = "FreeImage", "FreeImage Public License v1.0";
    Fsfap = "FSFAP", "FSF All Permissive License";
    Fsful = "FSFUL", "FSF Unlimited License";
    Fsfullr = "FSFULLR", "FSF Unlimited License (with License Retention)";
    Ftl = "FTL", "Freetype Project License";
    Gfdl1_1Only = "GFDL-1.1-only", "GNU Free Documentation License v1.1 only";
    Gfdl1_1OrLater = "GFDL-1.1-or-later", "GNU Free Documentation License v1.1 or later";
    Gfdl1_2Only = "GFDL-1.2-only", "GNU Free Documentation License v1.2 only";
    Gfdl1_2OrLater = "GFDL-1.2-or-later", "GNU Free Documentation License v1.2 or later";
    Gfdl1_3Only = "GFDL-1.3-only", "GNU Free Documentation License v1.3 only";
    Gfdl1_3OrLater = "GFDL-1.3-or-later", "GNU Free Documentation License v1.3 or later";
    Giftware = "Giftware", "Giftware License";
    GL2Ps = "GL2PS", "GL2PS License";
    Glide = "Glide", "3dfx Glide License";
    Glulxe = "Glulxe", "Glulxe License";
    Gnuplot = "gnuplot", "gnuplot License";
    Gpl1Only = "GPL-1.0-only", "GNU General Public License v1.0 only";
    Gpl1OrLater = "GPL-1.0-or-later", "GNU General Public License v1.0 or later";
    Gpl2Only = "GPL-2.0-only", "GNU General Public License v2.0 only";
    Gpl2OrLater = "GPL-2.0-or-later", "GNU General Public License v2.0 or later";
    Gpl3Only = "GPL-3.0-only", "GNU General Public License v3.0 only";
    Gpl3OrLater = "GPL-3.0-or-later", "GNU General Public License v3.0 or later";
    GSoap1_3b = "gSOAP-1.3b", "gSOAP Public License v1.3b";
    HaskellReport = "HaskellReport", "Haskell Language Report License";
    Hpnd = "HPND", "Historical Permission Notice and Disclaimer";
    HpndSellVariant = "HPND-sell-variant", "Historical Permission Notice and Disclaimer - sell variant";
    IbmPibs = "IBM-pibs", "IBM PowerPC Initialization and Boot Software";
    Icu = "ICU", "ICU License";
    Ijg = "IJG", "Independent JPEG Group License";
    ImageMagick = "ImageMagick", "ImageMagick License";
    IMatix = "iMatix", "iMatix Standard Function Library Agreement";
    Imlib2 = "Imlib2", "Imlib2 License";
    InfoZip = "Info-ZIP", "Info-ZIP License";
    Intel = "Intel", "Intel Open Source License";
    IntelAcpi = "Intel-ACPI", "Intel ACPI Software License Agreement";
    Interbase1 = "Interbase-1.0", "Interbase Public License v1.0";
    Ipa = "IPA", "IPA Font License";
    Ipl1 = "IPL-1.0", "IBM Public License v1.0";
    Isc = "ISC", "ISC License";
    JasPer2 = "JasPer-2.0", "JasPer License";
    Jpnic = "JPNIC", "Japan Network Information Center License";
    Json = "JSON", "JSON License";
    Lal1_2 = "LAL-1.2", "Licence Art Libre 1.2";
    Lal1_3 = "LAL-1.3", "Licence Art Libre 1.3";
    Latex2e = "Latex2e", "Latex2e License";
    Leptonica = "Leptonica", "Leptonica License";
    Lgpl2Only = "LGPL-2.0-only", "GNU Library General Public License v2 only";
    Lgpl2OrLater = "LGPL-2.0-or-later", "GNU Library General Public License v2 or later";
    Lgpl2_1Only = "LGPL-2.1-only", "GNU Lesser General Public License v2.1 only";
    Lgpl2_1OrLater = "LGPL-2.1-or-later", "GNU Lesser General Public License v2.1 or later";
    Lgpl3Only = "LGPL-3.0-only", "GNU Lesser General Public License v3.0 only";
    Lgpl3OrLater = "LGPL-3.0-or-later", "GNU Lesser General Public License v3.0 or later";
    Lgpllr = "LGPLLR", "Lesser General Public License For Linguistic Resources";
    Libpng = "Libpng", "libpng License";
    Libpng2 = "libpng-2.0", "PNG Reference Library version 2";
    Libtiff = "libtiff", "libtiff License";
    LiLiQP1_1 = "LiLiQ-P-1.1", "Licence Libre du Québec – Permissive version 1.1";
    LiLiQR1_1 = "LiLiQ-R-1.1", "Licence Libre du Québec – Réciprocité version 1.1";
    LiLiQRplus1_1 = "LiLiQ-Rplus-1.1", "Licence Libre du Québec – Réciprocité forte version 1.1";
    LinuxOpenIb = "Linux-OpenIB", "Linux Kernel Variant of OpenIB.org license";
    Lpl1 = "LPL-1.0", "Lucent Public License Version 1.0";
    Lpl1_02 = "LPL-1.02", "Lucent Public License v1.02";
    Lppl1 = "LPPL-1.0", "LaTeX Project Public License v1.0";
    Lppl1_1 = "LPPL-1.1", "LaTeX Project Public License v1.1";
    Lppl1_2 = "LPPL-1.2", "LaTeX Project Public License v1.2";
    Lppl1_3a = "LPPL-1.3a", "LaTeX Project Public License v1.3a";
    Lppl1_3c = "LPPL-1.3c", "LaTeX Project Public License v1.3c";
    MakeIndex = "MakeIndex", "MakeIndex License";
    MirOs = "MirOS", "The MirOS Licence";
    Mit = "MIT", "MIT License";
    Mit0 = "MIT-0", "MIT No Attribution";
    MitAdvertising = "MIT-advertising", "Enlightenment License (e16)";
    MitCmu = "MIT-CMU", "CMU License";
    MitEnna = "MIT-enna", "enna License";
    MitFeh = "MIT-feh", "feh License";
    Mitnfa = "MITNFA", "MIT +no-false-attribs license";
    Motosoto = "Motosoto", "Motosoto License";
    Mpich2 = "mpich2", "mpich2 License";
    Mpl1 = "MPL-1.0", "Mozilla Public License 1.0";
    Mpl1_1 = "MPL-1.1", "Mozilla Public License 1.1";
    Mpl2 = "MPL-2.0", "Mozilla Public License 2.0";
    Mpl2NoCopyleftException = "MPL-2.0-no-copyleft-exception", "Mozilla Public License 2.0 (no copyleft exception)";
    MsPl = "MS-PL", "Microsoft Public License";
    MsRl = "MS-RL", "Microsoft Reciprocal License";
    Mtll = "MTLL", "Matrix Template Library License";
    MulanPsl1 = "MulanPSL-1.0", "Mulan Permissive Software License, Version 1";
    Multics = "Multics", "Multics License";
    Mup = "Mup", "Mup License";
    Nasa1_3 = "NASA-1.3", "NASA Open Source Agreement 1.3";
    Naumen = "Naumen", "Naumen Public License";
    Nbpl1 = "NBPL-1.0", "Net Boolean Public License v1";
    Ncsa = "NCSA", "University of Illinois/NCSA Open Source License";
    NetSnmp = "Net-SNMP", "Net-SNMP License";
    NetCdf = "NetCDF", "NetCDF license";
    Newsletr = "Newsletr", "Newsletr License";
    Ngpl = "NGPL", "Nethack General Public License";
    Nlod1 = "NLOD-1.0", "Norwegian Licence for Open Government Data";
    Nlpl = "NLPL", "No Limit Public License";
    Nokia = "Nokia", "Nokia Open Source License";
    Nosl = "NOSL", "Netizen Open Source License";
    Noweb = "Noweb", "Noweb License";
    Npl1 = "NPL-1.0", "Netscape Public License v1.0";
    Npl1_1 = "NPL-1.1", "Netscape Public License v1.1";
    Nposl3 = "NPOSL-3.0", "Non-Profit Open Software License 3.0";
    Nrl = "NRL", "NRL License";
    Ntp = "NTP", "NTP License";
    OcctPl = "OCCT-PL", "Open CASCADE Technology Public License";
    Oclc2 = "OCLC-2.0", "OCLC Research Public License 2.0";
    ODbL1 = "ODbL-1.0", "ODC Open Database License v1.0";
    OdcBy1 = "ODC-By-1.0", "Open Data Commons Attribution License v1.0";
    Ofl1 = "OFL-1.0", "SIL Open Font License 1.0";
    Ofl1_1 = "OFL-1.1", "SIL Open Font License 1.1";
    OglCanada2 = "OGL-Canada-2.0", "Open Government Licence - Canada";
    OglUk1 = "OGL-UK-1.0", "Open Government Licence v1.0";
    OglUk2 = "OGL-UK-2.0", "Open Government Licence v2.0";
    OglUk3 = "OGL-UK-3.0", "Open Government Licence v3.0";
    Ogtsl = "OGTSL", "Open Group Test Suite License";
    Oldap1_1 = "OLDAP-1.1", "Open LDAP Public License v1.1";
    Oldap1_2 = "OLDAP-1.2", "Open LDAP Public License v1.2";
    Oldap1_3 = "OLDAP-1.3", "Open LDAP Public License v1.3";
    Oldap1_4 = "OLDAP-1.4", "Open LDAP Public License v1.4";
    Oldap2 = "OLDAP-2.0", "Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)";
    Oldap2_0_1 = "OLDAP-2.0.1", "Open LDAP Public License v2.0.1";
    Oldap2_1 = "OLDAP-2.1", "Open LDAP Public License v2.1";
    Oldap2_2 = "OLDAP-2.2", "Open LDAP Public License v2.2";
    Oldap2_2_1 = "OLDAP-2.2.1", "Open LDAP Public License v2.2.1";
    Oldap2_2_2 = "OLDAP-2.2.2", "Open LDAP Public License 2.2.2";
    Oldap2_3 = "OLDAP-2.3", "Open LDAP Public License v2.3";
    Oldap2_4 = "OLDAP-2.4", "Open LDAP Public License v2.4";
    Oldap2_5 = "OLDAP-2.5", "Open LDAP Public License v2.5";
    Oldap2_6 = "OLDAP-2.6", "Open LDAP Public License v2.6";
    Oldap2_7 = "OLDAP-2.7", "Open LDAP Public License v2.7";
    Oldap2_8 = "OLDAP-2.8", "Open LDAP Public License v2.8";
    Oml = "OML", "Open Market License";
    OpenSsl = "OpenSSL", "OpenSSL License";
    Opl1 = "OPL-1.0", "Open Public License v1.0";
    OsetPl2_1 = "OSET-PL-2.1", "OSET Public License version 2.1";
    Osl1 = "OSL-1.0", "Open Software License 1.0";
    Osl1_1 = "OSL-1.1", "Open Software License 1.1";
    Osl2 = "OSL-2.0", "Open Software License 2.0";
    Osl2_1 = "OSL-2.1", "Open Software License 2.1";
    Osl3 = "OSL-3.0", "Open Software License 3.0";
    Parity6 = "Parity-6.0.0", "The Parity Public License 6.0.0";
    Pddl1 = "PDDL-1.0", "ODC Public Domain Dedication & License 1.0";
    Php3 = "-PHP 3.0", "PHP License v3.0";
    Php3_01 = "-PHP 3.01", "PHP License v3.01";
    Plexus = "Plexus", "Plexus Classworlds License";
    PostgreSql = "PostgreSQL", "PostgreSQL License";
    Psfrag = "psfrag", "psfrag License";
    Psutils = "psutils", "psutils License";
    Python2 = "Python-2.0", "Python License 2.0";
    Qhull = "Qhull", "Qhull License";
    Qpl1 = "QPL-1.0", "Q Public License 1.0";
    Rdisc = "Rdisc", "Rdisc License";
    RHeCos1_1 = "RHeCos-1.1", "Red Hat eCos Public License v1.1";
    Rpl1_1 = "RPL-1.1", "Reciprocal Public License 1.1";
    Rpl1_5 = "RPL-1.5", "Reciprocal Public License 1.5";
    Rpsl1 = "RPSL-1.0", "RealNetworks Public Source License v1.0";
    RsaMd = "RSA-MD", "RSA Message-Digest License";
    Rscpl = "RSCPL", "Ricoh Source Code Public License";
    Ruby = "Ruby", "Ruby License";
    SaxPd = "SAX-PD", "Sax Public Domain Notice";
    Saxpath = "Saxpath", "Saxpath License";
    Scea = "SCEA", "SCEA Shared Source License";
    Sendmail = "Sendmail", "Sendmail License";
    Sendmail8_23 = "Sendmail-8.23", "Sendmail License 8.23";
    SgiB1 = "SGI-B-1.0", "SGI Free Software License B v1.0";
    SgiB1_1 = "SGI-B-1.1", "SGI Free Software License B v1.1";
    SgiB2 = "SGI-B-2.0", "SGI Free Software License B v2.0";
    Shl0_5 = "SHL-0.5", "Solderpad Hardware License v0.5";
    Shl0_51 = "SHL-0.51", "Solderpad Hardware License, Version 0.51";
    SimPl2 = "SimPL-2.0", "Simple Public License 2.0";
    Sissl = "SISSL", "Sun Industry Standards Source License v1.1";
    Sissl1_2 = "SISSL-1.2", "Sun Industry Standards Source License v1.2";
    Sleepycat = "Sleepycat", "Sleepycat License";
    Smlnj = "SMLNJ", "Standard ML of New Jersey License";
    Smppl = "SMPPL", "Secure Messaging Protocol Public License";
    Snia = "SNIA", "SNIA Public License 1.1";
    Spencer86 = "Spencer-86", "Spencer License 86";
    Spencer94 = "Spencer-94", "Spencer License 94";
    Spencer99 = "Spencer-99", "Spencer License 99";
    Spl1 = "SPL-1.0", "Sun Public License v1.0";
    SshOpenSsh = "SSH-OpenSSH", "SSH OpenSSH license";
    SshShort = "SSH-short", "SSH short notice";
    Sspl1 = "SSPL-1.0", "Server Side Public License, v 1";
    SugarCrm1_1_3 = "SugarCRM-1.1.3", "SugarCRM Public License v1.1.3";
    Swl = "SWL", "Scheme Widget Library (SWL) Software License Agreement";
    TaprOhl1 = "TAPR-OHL-1.0", "TAPR Open Hardware License v1.0";
    Tcl = "TCL", "TCL/TK License";
    TcpWrappers = "TCP-wrappers", "TCP Wrappers License";
    TMate = "TMate", "TMate Open Source License";
    Torque1_1 = "TORQUE-1.1", "TORQUE v2.5+ Software License v1.1";
    Tosl = "TOSL", "Trusster Open Source License";
    TuBerlin1 = "TU-Berlin-1.0", "Technische Universitaet Berlin License 1.0";
    TuBerlin2 = "TU-Berlin-2.0", "Technische Universitaet Berlin License 2.0";
    Ucl1 = "UCL-1.0", "Upstream Compatibility License v1.0";
    UnicodeDfs2015 = "Unicode-DFS-2015", "Unicode License Agreement - Data Files and Software (2015)";
    UnicodeDfs2016 = "Unicode-DFS-2016", "Unicode License Agreement - Data Files and Software (2016)";
    UnicodeTou = "Unicode-TOU", "Unicode Terms of Use";
    Unlicense = "Unlicense", "The Unlicense";
    Upl1 = "UPL-1.0", "Universal Permissive License v1.0";
    Vim = "Vim", "Vim License";
    Vostrom = "VOSTROM", "VOSTROM Public License for Open Source";
    Vsl1 = "VSL-1.0", "Vovida Software License v1.0";
    W3C = "W3C", "W3C Software Notice and License (2002-12-31)";
    W3C19980720 = "W3C-19980720", "W3C Software Notice and License (1998-07-20)";
    W3C20150513 = "W3C-20150513", "W3C Software Notice and Document License (2015-05-13)";
    Watcom1 = "Watcom-1.0", "Sybase Open Watcom Public License 1.0";
    Wsuipa = "Wsuipa", "Wsuipa License";
    Wtfpl = "WTFPL", "Do What The F*ck You Want To Public License";
    X11 = "X11", "X11 License";
    Xerox = "Xerox", "Xerox License";
    XFree861_1 = "XFree86-1.1", "XFree86 License 1.1";
    Xinetd = "xinetd", "xinetd License";
    Xnet = "Xnet", "X.Net License";
    Xpp = "xpp", "XPP License";
    XSkat = "XSkat", "XSkat License";
    Ypl1 = "YPL-1.0", "Yahoo! Public License v1.0";
    Ypl1_1 = "YPL-1.1", "Yahoo! Public License v1.1";
    Zed = "Zed", "Zed License";
    Zend2 = "Zend-2.0", "Zend License v2.0";
    Zimbra1_3 = "Zimbra-1.3", "Zimbra Public License v1.3";
    Zimbra1_4 = "Zimbra-1.4", "Zimbra Public License v1.4";
    Zlib = "Zlib", "zlib License";
    ZlibAcknowledgement = "zlib-acknowledgement", "zlib/libpng License with Acknowledgement";
    Zpl1_1 = "ZPL-1.1", "Zope Public License 1.1";
    Zpl2 = "ZPL-2.0", "Zope Public License 2.0";
    Zpl2_1 = "ZPL-2.1", "Zope Public License 2.1";
}

// A fixed-size array for indexing with a `SpdxLicense` casted to `usize`.
//
// Not publicly exported since
type Map<A> = [A; SpdxLicense::COUNT];

impl<'a> TryFrom<&'a str> for SpdxLicense {
    type Error = ParseError<'a>;

    #[inline]
    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        if id.is_empty() {
            return Err(ParseError::Empty);
        }
        LICENSE_BY_ID.get(id)
            .map(|&license| license)
            .ok_or(ParseError::UnknownLicenseId(id))
    }
}

impl fmt::Display for SpdxLicense {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id().fmt(f)
    }
}

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = SpdxLicense;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a known license")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        SpdxLicense::parse(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SpdxLicense {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Serialize for SpdxLicense {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.id())
    }
}

impl SpdxLicense {
    /// The current number of SPDX licenses.
    ///
    /// This number is allowed to change between otherwise API-compatible
    /// versions.
    // Defined here in order to be placed in docs alongside other items
    pub const COUNT: usize = Self::_COUNT;

    /// Returns an iterator over all licenses.
    ///
    /// ```
    /// use ocean::drop::license::SpdxLicense;
    ///
    /// let licenses = SpdxLicense::all();
    /// assert_eq!(licenses.len(), SpdxLicense::COUNT);
    /// ```
    #[inline]
    pub fn all() -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator {
        (0..(Self::COUNT as u16)).map(|l| unsafe { std::mem::transmute(l) })
    }

    /// Attempts to parse `input` and returns a [`ParseError`] on error.
    #[inline]
    pub fn parse<'a, I>(input: I) -> Result<Self, ParseError<'a>>
        where I: TryInto<Self, Error = ParseError<'a>>
    {
        input.try_into()
    }

    /// Returns the string identifier of this license.
    #[inline]
    pub const fn id(&self) -> &'static str {
        ID_BY_LICENSE[*self as usize]
    }

    /// Returns the full name of this license.
    #[inline]
    pub const fn name(&self) -> &'static str {
        NAME_BY_LICENSE[*self as usize]
    }
}

/// An error returned when attempting to parse a [`SpdxLicense`] or [`Expr`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    /// Empty string provided.
    Empty,
    /// An error returned when a license name is unknown.
    UnknownLicenseId(&'a str),
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => {
                write!(f, "empty string provided")
            },
            ParseError::UnknownLicenseId(id) => {
                write!(f, "'{}' is not a known license ID", id)
            },
        }
    }
}
