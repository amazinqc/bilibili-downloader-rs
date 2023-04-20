use std::sync::atomic::AtomicUsize;

pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15";
pub const COOKIE: &str = "CURRENT_FNVAL=4048; innersign=1; b_lsid=4C5D10AE2_18793225EED; bp_video_offset_32280488=785774145476493300; home_feed_column=4; FEED_LIVE_VERSION=V8; buvid4=2CDBDCD3-F6FF-B411-126E-BD175A161B8579338-022122219-0JGgJAvBL8TSMtvBMBpNoA%3D%3D; header_theme_version=CLOSE; i-wanna-go-back=-1; PVID=2; buvid_fp=dc35292e2c879f33da92c570dad80075; buvid_fp_plain=undefined; fingerprint=dc35292e2c879f33da92c570dad80075; CURRENT_BLACKGAP=0; is-2022-channel=1; CURRENT_PID=2e802d90-ca75-11ed-8830-7f48e1eb993c; CURRENT_QUALITY=112; hit-dyn-v2=1; hit-new-style-dyn=0; _uuid=1EB2A6D5-8318-10F63-E5C5-33BE45724DBC56564infoc; DedeUserID=32280488; DedeUserID__ckMd5=2ae085b4230bcd66; SESSDATA=aa1e6179%2C1687154862%2C5de75%2Ac2; bili_jct=05ab986191069c8db93ced776ad9b343; sid=8g6487kc; LIVE_BUVID=AUTO9516695486767921; rpdid=|(J~JYku|uuY0J'uYY)Y)kJ)|; b_ut=5; nostalgia_conf=-1; b_nut=1663776049; buvid3=1FF87ED7-2D57-84BE-3FD0-8F03EC5B47F949754infoc";
pub const PARTS: usize = 4;
pub(crate) static FINISHED: AtomicUsize = AtomicUsize::new(0);
pub(crate) static TOTAL: AtomicUsize = AtomicUsize::new(0);
