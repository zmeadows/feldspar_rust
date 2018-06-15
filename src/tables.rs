#![allow(dead_code)]

use core::*;
use bitboard::*;

pub const RANK1: Bitboard = Bitboard::new(255);
pub const RANK2: Bitboard = Bitboard::new(65280);
pub const RANK3: Bitboard = Bitboard::new(16711680);
pub const RANK4: Bitboard = Bitboard::new(4278190080);
pub const RANK5: Bitboard = Bitboard::new(1095216660480);
pub const RANK6: Bitboard = Bitboard::new(280375465082880);
pub const RANK7: Bitboard = Bitboard::new(71776119061217280);
pub const RANK8: Bitboard = Bitboard::new(18374686479671623680);

// pub const FILE1: Bitboard = Bitboard::new(72340172838076673);
// pub const FILE2: Bitboard = Bitboard::new(144680345676153346);
// pub const FILE3: Bitboard = Bitboard::new(289360691352306692);
// pub const FILE4: Bitboard = Bitboard::new(578721382704613384);
// pub const FILE5: Bitboard = Bitboard::new(1157442765409226768);
// pub const FILE6: Bitboard = Bitboard::new(2314885530818453536);
// pub const FILE7: Bitboard = Bitboard::new(4629771061636907072);
// pub const FILE8: Bitboard = Bitboard::new(9259542123273814144);

pub const WHITE_KINGSIDE_CASTLE_BITS: Bitboard = Bitboard::new(1 << 1 | 1 << 2);
pub const BLACK_KINGSIDE_CASTLE_BITS: Bitboard = Bitboard::new(1 << 63 - 6 | 1 << 63 - 5);

pub const BLACK_QUEENSIDE_CASTLE_BITS: Bitboard = Bitboard::new(1 << 63 - 1 | 1 << 63 - 2 | 1 << 63 - 3);
pub const WHITE_QUEENSIDE_CASTLE_BITS: Bitboard = Bitboard::new(1 << 4 | 1 << 5 | 1 << 6);

pub const BLACK_QUEENSIDE_CASTLE_SAFETY_BITS: Bitboard = Bitboard::new(1 << 63 - 2 | 1 << 63 - 3);
pub const WHITE_QUEENSIDE_CASTLE_SAFETY_BITS: Bitboard = Bitboard::new(1 << 4 | 1 << 5);

pub const KNIGHT_TABLE: [Bitboard; 64] =
  [ Bitboard::new(132096)
  , Bitboard::new(329728)
  , Bitboard::new(659712)
  , Bitboard::new(1319424)
  , Bitboard::new(2638848)
  , Bitboard::new(5277696)
  , Bitboard::new(10489856)
  , Bitboard::new(4202496)
  , Bitboard::new(33816580)
  , Bitboard::new(84410376)
  , Bitboard::new(168886289)
  , Bitboard::new(337772578)
  , Bitboard::new(675545156)
  , Bitboard::new(1351090312)
  , Bitboard::new(2685403152)
  , Bitboard::new(1075839008)
  , Bitboard::new(8657044482)
  , Bitboard::new(21609056261)
  , Bitboard::new(43234889994)
  , Bitboard::new(86469779988)
  , Bitboard::new(172939559976)
  , Bitboard::new(345879119952)
  , Bitboard::new(687463207072)
  , Bitboard::new(275414786112)
  , Bitboard::new(2216203387392)
  , Bitboard::new(5531918402816)
  , Bitboard::new(11068131838464)
  , Bitboard::new(22136263676928)
  , Bitboard::new(44272527353856)
  , Bitboard::new(88545054707712)
  , Bitboard::new(175990581010432)
  , Bitboard::new(70506185244672)
  , Bitboard::new(567348067172352)
  , Bitboard::new(1416171111120896)
  , Bitboard::new(2833441750646784)
  , Bitboard::new(5666883501293568)
  , Bitboard::new(11333767002587136)
  , Bitboard::new(22667534005174272)
  , Bitboard::new(45053588738670592)
  , Bitboard::new(18049583422636032)
  , Bitboard::new(145241105196122112)
  , Bitboard::new(362539804446949376)
  , Bitboard::new(725361088165576704)
  , Bitboard::new(1450722176331153408)
  , Bitboard::new(2901444352662306816)
  , Bitboard::new(5802888705324613632)
  , Bitboard::new(11533718717099671552)
  , Bitboard::new(4620693356194824192)
  , Bitboard::new(288234782788157440)
  , Bitboard::new(576469569871282176)
  , Bitboard::new(1224997833292120064)
  , Bitboard::new(2449995666584240128)
  , Bitboard::new(4899991333168480256)
  , Bitboard::new(9799982666336960512)
  , Bitboard::new(1152939783987658752)
  , Bitboard::new(2305878468463689728)
  , Bitboard::new(1128098930098176)
  , Bitboard::new(2257297371824128)
  , Bitboard::new(4796069720358912)
  , Bitboard::new(9592139440717824)
  , Bitboard::new(19184278881435648)
  , Bitboard::new(38368557762871296)
  , Bitboard::new(4679521487814656)
  , Bitboard::new(9077567998918656)
  ];

pub const KING_TABLE: [Bitboard; 64] =
    [ Bitboard::new(770)
    , Bitboard::new(1797)
    , Bitboard::new(3594)
    , Bitboard::new(7188)
    , Bitboard::new(14376)
    , Bitboard::new(28752)
    , Bitboard::new(57504)
    , Bitboard::new(49216)
    , Bitboard::new(197123)
    , Bitboard::new(460039)
    , Bitboard::new(920078)
    , Bitboard::new(1840156)
    , Bitboard::new(3680312)
    , Bitboard::new(7360624)
    , Bitboard::new(14721248)
    , Bitboard::new(12599488)
    , Bitboard::new(50463488)
    , Bitboard::new(117769984)
    , Bitboard::new(235539968)
    , Bitboard::new(471079936)
    , Bitboard::new(942159872)
    , Bitboard::new(1884319744)
    , Bitboard::new(3768639488)
    , Bitboard::new(3225468928)
    , Bitboard::new(12918652928)
    , Bitboard::new(30149115904)
    , Bitboard::new(60298231808)
    , Bitboard::new(120596463616)
    , Bitboard::new(241192927232)
    , Bitboard::new(482385854464)
    , Bitboard::new(964771708928)
    , Bitboard::new(825720045568)
    , Bitboard::new(3307175149568)
    , Bitboard::new(7718173671424)
    , Bitboard::new(15436347342848)
    , Bitboard::new(30872694685696)
    , Bitboard::new(61745389371392)
    , Bitboard::new(123490778742784)
    , Bitboard::new(246981557485568)
    , Bitboard::new(211384331665408)
    , Bitboard::new(846636838289408)
    , Bitboard::new(1975852459884544)
    , Bitboard::new(3951704919769088)
    , Bitboard::new(7903409839538176)
    , Bitboard::new(15806819679076352)
    , Bitboard::new(31613639358152704)
    , Bitboard::new(63227278716305408)
    , Bitboard::new(54114388906344448)
    , Bitboard::new(216739030602088448)
    , Bitboard::new(505818229730443264)
    , Bitboard::new(1011636459460886528)
    , Bitboard::new(2023272918921773056)
    , Bitboard::new(4046545837843546112)
    , Bitboard::new(8093091675687092224)
    , Bitboard::new(16186183351374184448)
    , Bitboard::new(13853283560024178688)
    , Bitboard::new(144959613005987840)
    , Bitboard::new(362258295026614272)
    , Bitboard::new(724516590053228544)
    , Bitboard::new(1449033180106457088)
    , Bitboard::new(2898066360212914176)
    , Bitboard::new(5796132720425828352)
    , Bitboard::new(11592265440851656704)
    , Bitboard::new(4665729213955833856)
    ];

pub const RAY_TABLE: [[Bitboard;64];8] =
[
  /* NORTH */
  [ Bitboard::new(72340172838076672)
  , Bitboard::new(144680345676153344)
  , Bitboard::new(289360691352306688)
  , Bitboard::new(578721382704613376)
  , Bitboard::new(1157442765409226752)
  , Bitboard::new(2314885530818453504)
  , Bitboard::new(4629771061636907008)
  , Bitboard::new(9259542123273814016)
  , Bitboard::new(72340172838076416)
  , Bitboard::new(144680345676152832)
  , Bitboard::new(289360691352305664)
  , Bitboard::new(578721382704611328)
  , Bitboard::new(1157442765409222656)
  , Bitboard::new(2314885530818445312)
  , Bitboard::new(4629771061636890624)
  , Bitboard::new(9259542123273781248)
  , Bitboard::new(72340172838010880)
  , Bitboard::new(144680345676021760)
  , Bitboard::new(289360691352043520)
  , Bitboard::new(578721382704087040)
  , Bitboard::new(1157442765408174080)
  , Bitboard::new(2314885530816348160)
  , Bitboard::new(4629771061632696320)
  , Bitboard::new(9259542123265392640)
  , Bitboard::new(72340172821233664)
  , Bitboard::new(144680345642467328)
  , Bitboard::new(289360691284934656)
  , Bitboard::new(578721382569869312)
  , Bitboard::new(1157442765139738624)
  , Bitboard::new(2314885530279477248)
  , Bitboard::new(4629771060558954496)
  , Bitboard::new(9259542121117908992)
  , Bitboard::new(72340168526266368)
  , Bitboard::new(144680337052532736)
  , Bitboard::new(289360674105065472)
  , Bitboard::new(578721348210130944)
  , Bitboard::new(1157442696420261888)
  , Bitboard::new(2314885392840523776)
  , Bitboard::new(4629770785681047552)
  , Bitboard::new(9259541571362095104)
  , Bitboard::new(72339069014638592)
  , Bitboard::new(144678138029277184)
  , Bitboard::new(289356276058554368)
  , Bitboard::new(578712552117108736)
  , Bitboard::new(1157425104234217472)
  , Bitboard::new(2314850208468434944)
  , Bitboard::new(4629700416936869888)
  , Bitboard::new(9259400833873739776)
  , Bitboard::new(72057594037927936)
  , Bitboard::new(144115188075855872)
  , Bitboard::new(288230376151711744)
  , Bitboard::new(576460752303423488)
  , Bitboard::new(1152921504606846976)
  , Bitboard::new(2305843009213693952)
  , Bitboard::new(4611686018427387904)
  , Bitboard::new(9223372036854775808)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  ],

  /* SOUTH */
  [ Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(1)
  , Bitboard::new(2)
  , Bitboard::new(4)
  , Bitboard::new(8)
  , Bitboard::new(16)
  , Bitboard::new(32)
  , Bitboard::new(64)
  , Bitboard::new(128)
  , Bitboard::new(257)
  , Bitboard::new(514)
  , Bitboard::new(1028)
  , Bitboard::new(2056)
  , Bitboard::new(4112)
  , Bitboard::new(8224)
  , Bitboard::new(16448)
  , Bitboard::new(32896)
  , Bitboard::new(65793)
  , Bitboard::new(131586)
  , Bitboard::new(263172)
  , Bitboard::new(526344)
  , Bitboard::new(1052688)
  , Bitboard::new(2105376)
  , Bitboard::new(4210752)
  , Bitboard::new(8421504)
  , Bitboard::new(16843009)
  , Bitboard::new(33686018)
  , Bitboard::new(67372036)
  , Bitboard::new(134744072)
  , Bitboard::new(269488144)
  , Bitboard::new(538976288)
  , Bitboard::new(1077952576)
  , Bitboard::new(2155905152)
  , Bitboard::new(4311810305)
  , Bitboard::new(8623620610)
  , Bitboard::new(17247241220)
  , Bitboard::new(34494482440)
  , Bitboard::new(68988964880)
  , Bitboard::new(137977929760)
  , Bitboard::new(275955859520)
  , Bitboard::new(551911719040)
  , Bitboard::new(1103823438081)
  , Bitboard::new(2207646876162)
  , Bitboard::new(4415293752324)
  , Bitboard::new(8830587504648)
  , Bitboard::new(17661175009296)
  , Bitboard::new(35322350018592)
  , Bitboard::new(70644700037184)
  , Bitboard::new(141289400074368)
  , Bitboard::new(282578800148737)
  , Bitboard::new(565157600297474)
  , Bitboard::new(1130315200594948)
  , Bitboard::new(2260630401189896)
  , Bitboard::new(4521260802379792)
  , Bitboard::new(9042521604759584)
  , Bitboard::new(18085043209519168)
  , Bitboard::new(36170086419038336)
  ],

  /* EAST */
  [ Bitboard::new(254)
  , Bitboard::new(252)
  , Bitboard::new(248)
  , Bitboard::new(240)
  , Bitboard::new(224)
  , Bitboard::new(192)
  , Bitboard::new(128)
  , Bitboard::new(0)
  , Bitboard::new(65024)
  , Bitboard::new(64512)
  , Bitboard::new(63488)
  , Bitboard::new(61440)
  , Bitboard::new(57344)
  , Bitboard::new(49152)
  , Bitboard::new(32768)
  , Bitboard::new(0)
  , Bitboard::new(16646144)
  , Bitboard::new(16515072)
  , Bitboard::new(16252928)
  , Bitboard::new(15728640)
  , Bitboard::new(14680064)
  , Bitboard::new(12582912)
  , Bitboard::new(8388608)
  , Bitboard::new(0)
  , Bitboard::new(4261412864)
  , Bitboard::new(4227858432)
  , Bitboard::new(4160749568)
  , Bitboard::new(4026531840)
  , Bitboard::new(3758096384)
  , Bitboard::new(3221225472)
  , Bitboard::new(2147483648)
  , Bitboard::new(0)
  , Bitboard::new(1090921693184)
  , Bitboard::new(1082331758592)
  , Bitboard::new(1065151889408)
  , Bitboard::new(1030792151040)
  , Bitboard::new(962072674304)
  , Bitboard::new(824633720832)
  , Bitboard::new(549755813888)
  , Bitboard::new(0)
  , Bitboard::new(279275953455104)
  , Bitboard::new(277076930199552)
  , Bitboard::new(272678883688448)
  , Bitboard::new(263882790666240)
  , Bitboard::new(246290604621824)
  , Bitboard::new(211106232532992)
  , Bitboard::new(140737488355328)
  , Bitboard::new(0)
  , Bitboard::new(71494644084506624)
  , Bitboard::new(70931694131085312)
  , Bitboard::new(69805794224242688)
  , Bitboard::new(67553994410557440)
  , Bitboard::new(63050394783186944)
  , Bitboard::new(54043195528445952)
  , Bitboard::new(36028797018963968)
  , Bitboard::new(0)
  , Bitboard::new(18302628885633695744)
  , Bitboard::new(18158513697557839872)
  , Bitboard::new(17870283321406128128)
  , Bitboard::new(17293822569102704640)
  , Bitboard::new(16140901064495857664)
  , Bitboard::new(13835058055282163712)
  , Bitboard::new(9223372036854775808)
  , Bitboard::new(0)
  ],

  /* WEST */
  [ Bitboard::new(0)
  , Bitboard::new(1)
  , Bitboard::new(3)
  , Bitboard::new(7)
  , Bitboard::new(15)
  , Bitboard::new(31)
  , Bitboard::new(63)
  , Bitboard::new(127)
  , Bitboard::new(0)
  , Bitboard::new(256)
  , Bitboard::new(768)
  , Bitboard::new(1792)
  , Bitboard::new(3840)
  , Bitboard::new(7936)
  , Bitboard::new(16128)
  , Bitboard::new(32512)
  , Bitboard::new(0)
  , Bitboard::new(65536)
  , Bitboard::new(196608)
  , Bitboard::new(458752)
  , Bitboard::new(983040)
  , Bitboard::new(2031616)
  , Bitboard::new(4128768)
  , Bitboard::new(8323072)
  , Bitboard::new(0)
  , Bitboard::new(16777216)
  , Bitboard::new(50331648)
  , Bitboard::new(117440512)
  , Bitboard::new(251658240)
  , Bitboard::new(520093696)
  , Bitboard::new(1056964608)
  , Bitboard::new(2130706432)
  , Bitboard::new(0)
  , Bitboard::new(4294967296)
  , Bitboard::new(12884901888)
  , Bitboard::new(30064771072)
  , Bitboard::new(64424509440)
  , Bitboard::new(133143986176)
  , Bitboard::new(270582939648)
  , Bitboard::new(545460846592)
  , Bitboard::new(0)
  , Bitboard::new(1099511627776)
  , Bitboard::new(3298534883328)
  , Bitboard::new(7696581394432)
  , Bitboard::new(16492674416640)
  , Bitboard::new(34084860461056)
  , Bitboard::new(69269232549888)
  , Bitboard::new(139637976727552)
  , Bitboard::new(0)
  , Bitboard::new(281474976710656)
  , Bitboard::new(844424930131968)
  , Bitboard::new(1970324836974592)
  , Bitboard::new(4222124650659840)
  , Bitboard::new(8725724278030336)
  , Bitboard::new(17732923532771328)
  , Bitboard::new(35747322042253312)
  , Bitboard::new(0)
  , Bitboard::new(72057594037927936)
  , Bitboard::new(216172782113783808)
  , Bitboard::new(504403158265495552)
  , Bitboard::new(1080863910568919040)
  , Bitboard::new(2233785415175766016)
  , Bitboard::new(4539628424389459968)
  , Bitboard::new(9151314442816847872)
  ],

  /* NORTHEAST */
  [ Bitboard::new(0)
  , Bitboard::new(256)
  , Bitboard::new(66048)
  , Bitboard::new(16909312)
  , Bitboard::new(4328785920)
  , Bitboard::new(1108169199616)
  , Bitboard::new(283691315109888)
  , Bitboard::new(72624976668147712)
  , Bitboard::new(0)
  , Bitboard::new(65536)
  , Bitboard::new(16908288)
  , Bitboard::new(4328783872)
  , Bitboard::new(1108169195520)
  , Bitboard::new(283691315101696)
  , Bitboard::new(72624976668131328)
  , Bitboard::new(145249953336262656)
  , Bitboard::new(0)
  , Bitboard::new(16777216)
  , Bitboard::new(4328521728)
  , Bitboard::new(1108168671232)
  , Bitboard::new(283691314053120)
  , Bitboard::new(72624976666034176)
  , Bitboard::new(145249953332068352)
  , Bitboard::new(290499906664136704)
  , Bitboard::new(0)
  , Bitboard::new(4294967296)
  , Bitboard::new(1108101562368)
  , Bitboard::new(283691179835392)
  , Bitboard::new(72624976397598720)
  , Bitboard::new(145249952795197440)
  , Bitboard::new(290499905590394880)
  , Bitboard::new(580999811180789760)
  , Bitboard::new(0)
  , Bitboard::new(1099511627776)
  , Bitboard::new(283673999966208)
  , Bitboard::new(72624942037860352)
  , Bitboard::new(145249884075720704)
  , Bitboard::new(290499768151441408)
  , Bitboard::new(580999536302882816)
  , Bitboard::new(1161999072605765632)
  , Bitboard::new(0)
  , Bitboard::new(281474976710656)
  , Bitboard::new(72620543991349248)
  , Bitboard::new(145241087982698496)
  , Bitboard::new(290482175965396992)
  , Bitboard::new(580964351930793984)
  , Bitboard::new(1161928703861587968)
  , Bitboard::new(2323857407723175936)
  , Bitboard::new(0)
  , Bitboard::new(72057594037927936)
  , Bitboard::new(144115188075855872)
  , Bitboard::new(288230376151711744)
  , Bitboard::new(576460752303423488)
  , Bitboard::new(1152921504606846976)
  , Bitboard::new(2305843009213693952)
  , Bitboard::new(4611686018427387904)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  ],

  /* NORTHWEST */
  [ Bitboard::new(9241421688590303744)
  , Bitboard::new(36099303471055872)
  , Bitboard::new(141012904183808)
  , Bitboard::new(550831656960)
  , Bitboard::new(2151686144)
  , Bitboard::new(8404992)
  , Bitboard::new(32768)
  , Bitboard::new(0)
  , Bitboard::new(4620710844295151616)
  , Bitboard::new(9241421688590303232)
  , Bitboard::new(36099303471054848)
  , Bitboard::new(141012904181760)
  , Bitboard::new(550831652864)
  , Bitboard::new(2151677952)
  , Bitboard::new(8388608)
  , Bitboard::new(0)
  , Bitboard::new(2310355422147510272)
  , Bitboard::new(4620710844295020544)
  , Bitboard::new(9241421688590041088)
  , Bitboard::new(36099303470530560)
  , Bitboard::new(141012903133184)
  , Bitboard::new(550829555712)
  , Bitboard::new(2147483648)
  , Bitboard::new(0)
  , Bitboard::new(1155177711056977920)
  , Bitboard::new(2310355422113955840)
  , Bitboard::new(4620710844227911680)
  , Bitboard::new(9241421688455823360)
  , Bitboard::new(36099303202095104)
  , Bitboard::new(141012366262272)
  , Bitboard::new(549755813888)
  , Bitboard::new(0)
  , Bitboard::new(577588851233521664)
  , Bitboard::new(1155177702467043328)
  , Bitboard::new(2310355404934086656)
  , Bitboard::new(4620710809868173312)
  , Bitboard::new(9241421619736346624)
  , Bitboard::new(36099165763141632)
  , Bitboard::new(140737488355328)
  , Bitboard::new(0)
  , Bitboard::new(288793326105133056)
  , Bitboard::new(577586652210266112)
  , Bitboard::new(1155173304420532224)
  , Bitboard::new(2310346608841064448)
  , Bitboard::new(4620693217682128896)
  , Bitboard::new(9241386435364257792)
  , Bitboard::new(36028797018963968)
  , Bitboard::new(0)
  , Bitboard::new(144115188075855872)
  , Bitboard::new(288230376151711744)
  , Bitboard::new(576460752303423488)
  , Bitboard::new(1152921504606846976)
  , Bitboard::new(2305843009213693952)
  , Bitboard::new(4611686018427387904)
  , Bitboard::new(9223372036854775808)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  ],

  /* SOUTHEAST */
  [ Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(1)
  , Bitboard::new(2)
  , Bitboard::new(4)
  , Bitboard::new(8)
  , Bitboard::new(16)
  , Bitboard::new(32)
  , Bitboard::new(64)
  , Bitboard::new(0)
  , Bitboard::new(256)
  , Bitboard::new(513)
  , Bitboard::new(1026)
  , Bitboard::new(2052)
  , Bitboard::new(4104)
  , Bitboard::new(8208)
  , Bitboard::new(16416)
  , Bitboard::new(0)
  , Bitboard::new(65536)
  , Bitboard::new(131328)
  , Bitboard::new(262657)
  , Bitboard::new(525314)
  , Bitboard::new(1050628)
  , Bitboard::new(2101256)
  , Bitboard::new(4202512)
  , Bitboard::new(0)
  , Bitboard::new(16777216)
  , Bitboard::new(33619968)
  , Bitboard::new(67240192)
  , Bitboard::new(134480385)
  , Bitboard::new(268960770)
  , Bitboard::new(537921540)
  , Bitboard::new(1075843080)
  , Bitboard::new(0)
  , Bitboard::new(4294967296)
  , Bitboard::new(8606711808)
  , Bitboard::new(17213489152)
  , Bitboard::new(34426978560)
  , Bitboard::new(68853957121)
  , Bitboard::new(137707914242)
  , Bitboard::new(275415828484)
  , Bitboard::new(0)
  , Bitboard::new(1099511627776)
  , Bitboard::new(2203318222848)
  , Bitboard::new(4406653222912)
  , Bitboard::new(8813306511360)
  , Bitboard::new(17626613022976)
  , Bitboard::new(35253226045953)
  , Bitboard::new(70506452091906)
  , Bitboard::new(0)
  , Bitboard::new(281474976710656)
  , Bitboard::new(564049465049088)
  , Bitboard::new(1128103225065472)
  , Bitboard::new(2256206466908160)
  , Bitboard::new(4512412933881856)
  , Bitboard::new(9024825867763968)
  , Bitboard::new(18049651735527937)
  ],

  /* SOUTHWEST */
  [ Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(2)
  , Bitboard::new(4)
  , Bitboard::new(8)
  , Bitboard::new(16)
  , Bitboard::new(32)
  , Bitboard::new(64)
  , Bitboard::new(128)
  , Bitboard::new(0)
  , Bitboard::new(516)
  , Bitboard::new(1032)
  , Bitboard::new(2064)
  , Bitboard::new(4128)
  , Bitboard::new(8256)
  , Bitboard::new(16512)
  , Bitboard::new(32768)
  , Bitboard::new(0)
  , Bitboard::new(132104)
  , Bitboard::new(264208)
  , Bitboard::new(528416)
  , Bitboard::new(1056832)
  , Bitboard::new(2113664)
  , Bitboard::new(4227072)
  , Bitboard::new(8388608)
  , Bitboard::new(0)
  , Bitboard::new(33818640)
  , Bitboard::new(67637280)
  , Bitboard::new(135274560)
  , Bitboard::new(270549120)
  , Bitboard::new(541097984)
  , Bitboard::new(1082130432)
  , Bitboard::new(2147483648)
  , Bitboard::new(0)
  , Bitboard::new(8657571872)
  , Bitboard::new(17315143744)
  , Bitboard::new(34630287488)
  , Bitboard::new(69260574720)
  , Bitboard::new(138521083904)
  , Bitboard::new(277025390592)
  , Bitboard::new(549755813888)
  , Bitboard::new(0)
  , Bitboard::new(2216338399296)
  , Bitboard::new(4432676798592)
  , Bitboard::new(8865353596928)
  , Bitboard::new(17730707128320)
  , Bitboard::new(35461397479424)
  , Bitboard::new(70918499991552)
  , Bitboard::new(140737488355328)
  , Bitboard::new(0)
  , Bitboard::new(567382630219904)
  , Bitboard::new(1134765260439552)
  , Bitboard::new(2269530520813568)
  , Bitboard::new(4539061024849920)
  , Bitboard::new(9078117754732544)
  , Bitboard::new(18155135997837312)
  , Bitboard::new(36028797018963968)
  , Bitboard::new(0)
  ]
];

pub const PAWN_ATTACKS: [[Bitboard;64];2] =
[
  [ Bitboard::new(512)
  , Bitboard::new(1280)
  , Bitboard::new(2560)
  , Bitboard::new(5120)
  , Bitboard::new(10240)
  , Bitboard::new(20480)
  , Bitboard::new(40960)
  , Bitboard::new(16384)
  , Bitboard::new(131072)
  , Bitboard::new(327680)
  , Bitboard::new(655360)
  , Bitboard::new(1310720)
  , Bitboard::new(2621440)
  , Bitboard::new(5242880)
  , Bitboard::new(10485760)
  , Bitboard::new(4194304)
  , Bitboard::new(33554432)
  , Bitboard::new(83886080)
  , Bitboard::new(167772160)
  , Bitboard::new(335544320)
  , Bitboard::new(671088640)
  , Bitboard::new(1342177280)
  , Bitboard::new(2684354560)
  , Bitboard::new(1073741824)
  , Bitboard::new(8589934592)
  , Bitboard::new(21474836480)
  , Bitboard::new(42949672960)
  , Bitboard::new(85899345920)
  , Bitboard::new(171798691840)
  , Bitboard::new(343597383680)
  , Bitboard::new(687194767360)
  , Bitboard::new(274877906944)
  , Bitboard::new(2199023255552)
  , Bitboard::new(5497558138880)
  , Bitboard::new(10995116277760)
  , Bitboard::new(21990232555520)
  , Bitboard::new(43980465111040)
  , Bitboard::new(87960930222080)
  , Bitboard::new(175921860444160)
  , Bitboard::new(70368744177664)
  , Bitboard::new(562949953421312)
  , Bitboard::new(1407374883553280)
  , Bitboard::new(2814749767106560)
  , Bitboard::new(5629499534213120)
  , Bitboard::new(11258999068426240)
  , Bitboard::new(22517998136852480)
  , Bitboard::new(45035996273704960)
  , Bitboard::new(18014398509481984)
  , Bitboard::new(144115188075855872)
  , Bitboard::new(360287970189639680)
  , Bitboard::new(720575940379279360)
  , Bitboard::new(1441151880758558720)
  , Bitboard::new(2882303761517117440)
  , Bitboard::new(5764607523034234880)
  , Bitboard::new(11529215046068469760)
  , Bitboard::new(4611686018427387904)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  ],

  [ Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(0)
  , Bitboard::new(2)
  , Bitboard::new(5)
  , Bitboard::new(10)
  , Bitboard::new(20)
  , Bitboard::new(40)
  , Bitboard::new(80)
  , Bitboard::new(160)
  , Bitboard::new(64)
  , Bitboard::new(512)
  , Bitboard::new(1280)
  , Bitboard::new(2560)
  , Bitboard::new(5120)
  , Bitboard::new(10240)
  , Bitboard::new(20480)
  , Bitboard::new(40960)
  , Bitboard::new(16384)
  , Bitboard::new(131072)
  , Bitboard::new(327680)
  , Bitboard::new(655360)
  , Bitboard::new(1310720)
  , Bitboard::new(2621440)
  , Bitboard::new(5242880)
  , Bitboard::new(10485760)
  , Bitboard::new(4194304)
  , Bitboard::new(33554432)
  , Bitboard::new(83886080)
  , Bitboard::new(167772160)
  , Bitboard::new(335544320)
  , Bitboard::new(671088640)
  , Bitboard::new(1342177280)
  , Bitboard::new(2684354560)
  , Bitboard::new(1073741824)
  , Bitboard::new(8589934592)
  , Bitboard::new(21474836480)
  , Bitboard::new(42949672960)
  , Bitboard::new(85899345920)
  , Bitboard::new(171798691840)
  , Bitboard::new(343597383680)
  , Bitboard::new(687194767360)
  , Bitboard::new(274877906944)
  , Bitboard::new(2199023255552)
  , Bitboard::new(5497558138880)
  , Bitboard::new(10995116277760)
  , Bitboard::new(21990232555520)
  , Bitboard::new(43980465111040)
  , Bitboard::new(87960930222080)
  , Bitboard::new(175921860444160)
  , Bitboard::new(70368744177664)
  , Bitboard::new(562949953421312)
  , Bitboard::new(1407374883553280)
  , Bitboard::new(2814749767106560)
  , Bitboard::new(5629499534213120)
  , Bitboard::new(11258999068426240)
  , Bitboard::new(22517998136852480)
  , Bitboard::new(45035996273704960)
  , Bitboard::new(18014398509481984)
  ]
];

pub fn get_positive_ray(square: Square, dir: Direction, mut occupied: Bitboard) -> Bitboard
{
   let attacks = RAY_TABLE[dir as usize][square.idx()];
   occupied &= attacks;
   if occupied.nonempty() {
      let blocker_square = occupied.bitscan_forward();
      return attacks ^ RAY_TABLE[dir as usize][blocker_square.idx()];
   } else {
       return attacks;
   }
}

pub fn get_negative_ray(square: Square, dir: Direction, mut occupied: Bitboard) -> Bitboard
{
   let attacks = RAY_TABLE[dir as usize][square.idx()];
   occupied &= attacks;
   // TODO: don't use nonemtpy and bitscan_reverse calls, bitscan_reverse is enough
   if occupied.nonempty() {
      let blocker_square = occupied.bitscan_reverse();
      return attacks ^ RAY_TABLE[dir as usize][blocker_square.idx()];
   } else {
       return attacks;
   }
}

pub fn get_diagonal_ray(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_positive_ray(square, Direction::NE, occupied)
         | get_negative_ray(square, Direction::SW, occupied);
}

pub fn get_antidiagonal_ray(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_positive_ray(square, Direction::NW, occupied)
         | get_negative_ray(square, Direction::SE, occupied);
}

pub fn get_file_ray(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_positive_ray(square, Direction::N, occupied)
         | get_negative_ray(square, Direction::S, occupied);
}

pub fn get_rank_ray(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_positive_ray(square, Direction::E, occupied)
         | get_negative_ray(square, Direction::W, occupied);
}

pub fn get_rook_rays(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_file_ray(square, occupied)
         | get_rank_ray(square, occupied);
}

pub fn get_bishop_rays(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_diagonal_ray(square, occupied)
         | get_antidiagonal_ray(square, occupied);
}

pub fn get_queen_rays(square: Square, occupied: Bitboard) -> Bitboard
{
    return get_bishop_rays(square, occupied)
         | get_rook_rays(square, occupied);
}

pub fn ray_between_squares(sq_a: Square, sq_b: Square) -> Bitboard
{
    //TODO: turn this into a lookup table
    let sqb_bit = sq_b.bitrep();

    let mut ray = get_positive_ray(sq_a, Direction::N, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_positive_ray(sq_a, Direction::E, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::S, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::W, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_positive_ray(sq_a, Direction::NE, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_positive_ray(sq_a, Direction::NW, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::SW, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::SE, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    return Bitboard::new(0);
}

pub fn diagonal_ray_between_squares(sq_a: Square, sq_b: Square) -> Bitboard
{
    //TODO: turn this into a lookup table
    let sqb_bit = sq_b.bitrep();

    let mut ray = get_positive_ray(sq_a, Direction::NE, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_positive_ray(sq_a, Direction::NW, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::SW, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::SE, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    return Bitboard::new(0);
}

pub fn nondiagonal_ray_between_squares(sq_a: Square, sq_b: Square) -> Bitboard
{
    //TODO: turn this into a lookup table
    let sqb_bit = sq_b.bitrep();

    let mut ray = get_positive_ray(sq_a, Direction::N, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_positive_ray(sq_a, Direction::E, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::S, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    ray = get_negative_ray(sq_a, Direction::W, sqb_bit);
    if (ray & sqb_bit).nonempty() { return ray; }

    return Bitboard::new(0);
}

pub fn xray_rook_attacks(occ: Bitboard, mut blockers: Bitboard, rook_square: Square) -> Bitboard {
   let attacks = get_rook_rays(rook_square, occ);
   blockers &= attacks;
   return attacks ^ get_rook_rays(rook_square, occ ^ blockers);
}

pub fn xray_bishop_attacks(occ: Bitboard, mut blockers: Bitboard, bishop_square: Square) -> Bitboard {
   let attacks = get_bishop_rays(bishop_square, occ);
   blockers &= attacks;
   return attacks ^ get_bishop_rays(bishop_square, occ ^ blockers);
}
