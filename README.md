# Sawshark

Sawshark is an annotation utility for sequence-resolved structural variants.

Sawshark was written as a companion annotation tool for the [sawfish SV
caller](https://github.com/PacificBiosciences/sawfish), so it is primarily tested for sawfish VCFs. It currently
provides only a limited SV annotation capability, meant to complement rather than replace annotation tooling such as
[svpack](https://github.com/PacificBiosciences/svpack).

Sawshark currently provides a mode for mobile-element annotations compatible with those from
[pbsv](https://github.com/PacificBiosciences/pbsv), adding ALU, L1, or SVA labels to the `INFO/SVANN` VCF field for
matching SVs. Sawshark can run from any SV VCF/BCF file or on a stdin VCF stream, and writes annotated VCF output to
stdout. The following command demonstrates typical usage:

```
sawshark --vcf genotyped.sv.vcf.gz --threads 8 | bcftools view - -Oz -o genotyped.sv.anno.vcf.gz
```

Note that sawshark will always preserve the input vcf record order in its VCF output.

### Installation

To install sawshark, download the latest release tarball compiled for 64-bit Linux on the [github release
channel](https://github.com/PacificBiosciences/sawshark/releases/latest), then unpack the tar file. Using v0.2.0 as an
example, the tar file can be downloaded and unpacked as follows:

    wget https://github.com/PacificBiosciences/sawshark/releases/download/v0.2.0/sawshark-v0.2.0-x86_64-unknown-linux-gnu.tar.gz
    tar -xzf sawshark-v0.2.0-x86_64-unknown-linux-gnu.tar.gz

The sawshark binary is found in the `bin/` directory of the unpacked file distribution. This can be run with the help
option to test the binary and review latest usage details:

    sawshark-v0.2.0-x86_64-unknown-linux-gnu/bin/sawshark --help

## Support

Create a new [issue ticket](https://github.com/PacificBiosciences/sawshark/issues) on this repo for support with current
capabilities or new feature requests.

## DISCLAIMER
THIS WEBSITE AND CONTENT AND ALL SITE-RELATED SERVICES, INCLUDING ANY DATA, ARE PROVIDED "AS IS," WITH ALL FAULTS, WITH
NO REPRESENTATIONS OR WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, ANY WARRANTIES
OF MERCHANTABILITY, SATISFACTORY QUALITY, NON-INFRINGEMENT OR FITNESS FOR A PARTICULAR PURPOSE. YOU ASSUME TOTAL
RESPONSIBILITY AND RISK FOR YOUR USE OF THIS SITE, ALL SITE-RELATED SERVICES, AND ANY THIRD PARTY WEBSITES OR
APPLICATIONS. NO ORAL OR WRITTEN INFORMATION OR ADVICE SHALL CREATE A WARRANTY OF ANY KIND. ANY REFERENCES TO SPECIFIC
PRODUCTS OR SERVICES ON THE WEBSITES DO NOT CONSTITUTE OR IMPLY A RECOMMENDATION OR ENDORSEMENT BY PACIFIC BIOSCIENCES.
