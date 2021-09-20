let
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/55a5eef58979fb6bc476d8c3e0c028cdeb1b5421.tar.gz";
    sha256 = "sha256:0q6d0rql1pyy93xw1c8s28jjjcgk1zgwxwixsp9z5r4w2ihaz3zg";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";

    holochainVersion = {
     rev = "3503b38abf4196ee8b21ba09c7a4e348794ce4d4";
     sha256 = "sha256:06khjy1n5l3aa8alakkbqiyhdgdcl64jzdbcf1pv6a15i1yvargq";
     cargoSha256 = "sha256:0bxflwmdh785c99cjgpmynd0h70a5gm40pryzzrfd9xiypr29gi7";
     bins = {
       holochain = "holochain";
       hc = "hc";
       kitsune-p2p-proxy = "kitsune_p2p/proxy";
     };
     lairKeystoreHashes = {
       sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
       cargoSha256 = "1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
     };
    };
  };
in holonix.main
