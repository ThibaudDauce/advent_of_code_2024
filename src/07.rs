fn main() {
    part1();
}

fn part1() {
    let mut sum = 0;
    for line in input().trim().lines() {
        let (goal, list) = line.trim().split_once(": ").unwrap();
        let goal: usize = goal.parse().unwrap();
        let values: Vec<usize> = list
            .split(' ')
            .map(|value| value.parse().unwrap())
            .collect();

        if can_match_goal(goal, 0, &values) {
            sum += goal;
        }
    }

    println!("Part 1 is {sum}");
}

fn can_match_goal(goal: usize, result: usize, values: &[usize]) -> bool {
    if values.is_empty() {
        return goal == result;
    }

    // if result >= goal && !values.contains(&0) {
    //     return false;
    // }

    return can_match_goal(goal, result + values[0], &values[1..])
        || can_match_goal(goal, result * values[0], &values[1..]);
}

fn example() -> &'static str {
    "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
    "
}

fn input() -> &'static str {
    "
5055314034: 10 1 489 35 44 14 16 4
108773: 18 1 121 2 6 38
22175: 1 73 45 2 35 5
4076637638: 338 1 6 1 191 7 3 4 34 4
479231839: 9 7 73 1 6 4 7 8 581 7 2 8
161312232: 761 852 11 4 8 28 6
6760600000: 220 40 7 5 878 25
601121975: 608 91 53 5 5 53 649
761: 79 2 6 558 42
84130: 65 90 8 6 9
52590888326: 7 3 9 3 8 543 1 673 8 4
21603: 9 80 3 1
6498240: 9 6 7 32 210
2420318475: 8 413 85 167 5 81
6368572316: 885 2 2 599 5 1 6 6 6 5 9
23428423: 870 8 1 308 581 5 34 9
17387502: 1 8 50 1 76 67 395
16073047995028: 9 8 2 6 2 282 5 507 954
1527631: 2 8 95 6 8 8 152 6 1 9 5 8
90086454: 32 593 2 1 4 1 6 4 6 45 7
516078606614: 8 60 6 7 860 59 7 14
145962: 2 86 847 269 3 6
81188: 9 8 7 3 386 37 82 2 8
159171: 50 1 764 817 12 4
9440342478: 866 326 50 499 67
177118: 77 846 845 3 18
389593323: 41 95 2 3 62 46 6 780 3
2318793: 3 74 9 8 792
6077575: 421 4 6 84 17 7
1513923281: 760 3 166 82 4 1
1368574: 950 17 520 460 2 534
2602638433: 6 506 58 1 346 262 4
324285404: 7 4 1 6 6 443 3 4 6 41 1 4
123419188: 831 275 4 1 8 9 7 9 5 6
7639222: 3 2 1 50 4 9 823 98 1 2 8
2165757141719: 93 575 4 8 498 81 5 5 6
32949456: 7 6 1 1 338 31 9 6 958 7
373015448: 7 1 8 86 146 2 8 6 545 5
58401: 50 1 74 9 7 3 672 7 59
321369974958: 3 5 380 7 3 3 2 5 801 3 9
1654884263287: 398 42 99 1 72 91 287
1497933556: 4 89 6 2 41 8 5 558
14538726514: 726 8 658 699 6 2 514
653959328: 63 519 95 2 3 27
12554226440: 8 4 46 8 8 912 828 58 7
24068829: 2 1 7 9 8 6 3 14 33 6 6 1
13716449: 1 37 16 449
70300416144: 1 9 61 300 406 2 8 1 44
12041328: 60 3 4 839 78
179748877650: 179 2 5 4 881 6 76 49
22666025380: 4 230 3 54 99 96 456 4
74741534: 747 4 14 33 40 61
194212880: 97 2 201 7 4 8 1 98 7 8 2
534529044: 7 7 673 8 4 680 7 9 28 9
688: 4 5 5 23 6
256747454: 8 3 6 9 9 3 76 266 9 5 2
61497991: 52 29 759 18 993
8046070: 4 35 2 833 36 70
19899933: 6 23 801 6 5 94 4 5 3 2 3
175019472155: 9 4 84 444 214 25 5 30
122: 2 12 8 5 5
7716862272: 8 6 1 9 1 9 782 49 8 4 3 7
292572: 7 3 314 36 21 1
9265739: 181 9 828 7 5 8 5 1 38 9
38166224456: 8 55 930 6 995 144 54
10633: 461 77 9 972 7
11448249: 11 359 89 2 51
6409: 3 2 5 6 252 8 4 3 352 3
144060624: 7 3 34 1 601 1 27 907 6
29213: 97 1 3 8 6
61614560278: 23 4 758 900 98 35 8
701009838: 786 93 959 2 13
170675: 51 1 4 446 34 27 6 5
3590981856: 2 6 77 7 3 6 6 12 99 77 7
159313258080: 20 933 158 9 79 76
13950095: 620 774 482 3 524 5
19276100: 3 828 1 95 773 519 53
91701711135: 58 300 704 2 527
19037065502: 29 69 9 1 90 5 46 955 9
4796722: 841 57 30 1 9
6491761244154: 9 689 19 5 83 5 68 67
2288734074: 3 20 351 81 11 35 7 2
51: 1 7 3
6703: 5 19 2 7 6 3 8 6 3 678 3 7
55285: 7 151 96 43 5 673
1345: 20 5 4 13 323 2
648683: 90 9 85 8
647351437: 1 79 2 8 7 568 12 2 3 5
496420640: 9 34 69 9 710 2 8
17707: 78 98 93 7 7
1322647: 1 6 4 7 4 2 8 8 9 8 50 283
13373109: 2 771 1 865 1 9 5 5 2 5 4
1771452188: 17 714 52 1 86
3876558518158: 3 6 5 6 9 2 3 4 271 5 53 8
121216672: 3 2 8 3 7 6 57 8 4 4 8 674
99709866569: 778 9 7 32 42 664 4 7
8132: 61 12 84 5 743
181628139355: 3 48 7 9 3 5 475 41 5 7 4
8137536: 2 3 5 7 8 3 7 31 7 2 6 9
125320123: 16 7 7 13 96 372 8 123
3928122: 7 3 5 97 4 4 6 389
16010345: 8 25 8 95 1 5 590 230 9
353033: 6 434 5 80 633
662454: 882 6 7 3 402 3 28 26
4623860913: 6 6 8 6 2 49 8 3 2 8 4 982
6223012578846: 70 889 125 788 46
1233252: 8 6 5 9 3 804 2 9 213 8 9
6377598: 7 5 7 11 692 3 2 9 3 1 2 6
851215538888: 9 5 4 7 4 1 9 42 8 2 890
652063806: 26 4 21 8 1 6 2 49 8 3 5 8
569651493608: 626 5 17 9 531 3 607 2
109564049: 5 4 4 2 622 5 501 5 50 1
108244890: 920 181 7 3 6 65
3064775: 6 3 1 7 1 6 7 9 65 9 581 5
2168098354: 4 479 73 6 40 4 29 53 6
2213354545: 9 1 212 335 38 87 656
113305647: 83 1 93 9 5 36 4 8 3
3648621: 45 2 9 6 4 3 4 1 76 1 7 6
39788761: 3 631 52 58 75 8
37988340: 996 882 389 3 52
105593616: 3 9 2 2 9 196 1 815 57 3
34732786878: 391 3 6 907 6 88 1
432317883: 273 791 11 182
485351797: 58 4 76 83 9 96
75514220688: 839 90 4 220 688
8989734402: 89 81 8 734 400 2
1560183: 7 48 54 4 1 84 1
1639799045: 116 589 8 9 2 5 3 8 8
52538: 29 78 3 736 62 86
943: 726 6 206 5 3
426657: 5 53 7 230 7
34098: 40 841 1 3 6 441 7
987900367: 6 4 6 8 98 8 2 6 36 4 1 3
3240: 7 2 16 6 9 81
167040619026129: 8 5 537 764 5 9 479 2 9
21896: 19 4 6 4 86 184
473: 364 1 95 7 6
231: 5 8 191
4095: 7 571 9 8 81
40921600: 20 37 92 816 4 50
39353184077: 99 835 3 9 1 585 8 79
224194610985: 9 552 4 190 3 999 2 9 2
81804625: 8 79 5 93 27 84 27
146: 9 6 8 15 9
793655568: 491 449 8 8 9 4
83314065: 462 8 559 18 6
364726: 40 87 27 26 4
1678449: 176 3 952 73
2423015: 5 76 99 6 8 8 501 3 8
171837368646: 94 1 609 33 1 3 68 648
686498: 8 8 78 95 2
504336092: 5 945 553 30 4 2 9 9 5 4
73304: 12 736 98
3458989639: 4 6 3 61 7 95 6 50 463 9
182: 1 4 90 83 5
2487563: 78 1 992 61 23 45
13836888716: 772 7 1 8 272 5 8 4 8 68
6288370: 29 7 180 77 7 137 207
10944097534206: 1 673 409 408 9 2 327
1195444320: 92 7 998 6 310
27360851258: 66 6 9 9 4 2 5 8 51 258
6317930: 986 63 70 9 982 30
68190208: 29 918 40 576 8 8 8 8
21470713: 1 7 2 3 200 5 2 11 359
309386: 3 72 3 1 6 41
88555437: 72 696 19 81 93
692826037754: 766 4 663 899 5 2 54
88288240327202: 6 6 65 539 57 61 7 20 4
6481223: 646 9 9 3 145 4 72
37018831358: 671 7 6 91 31 358
185508656: 2 14 9 371 4 14 5 93 1 4
60076539: 4 150 760 39 3 491 5
349331: 2 3 27 317 329
6527: 7 1 7 4 7 5 5 2 6 50 1 54
582802573: 321 516 6 284 2 245
12151573: 5 92 44 3 9 663 551 9 6
3265154989: 777 408 9 8 7 2 36 2 3 4
516041069541: 8 2 4 17 72 7 7 1 546 4
1695: 7 5 327 5
17171: 163 5 52 321
177861: 4 94 120 6 3
17432592: 9 870 573 716 12
420355623360: 1 213 7 23 98 19 33 61
572193: 9 26 3 902 655 6
417217: 46 2 27 2 58 83 4 55
2061628: 474 8 2 3 656 978 4
546001430: 6 8 391 637 8 80 22 65
281028055: 3 35 9 3 9 9 8 8 48 33 55
33580: 33 5 68 6 6
723642076: 467 7 973 2 8 412 5 18
47287138632892: 72 749 4 4 405 65 3 90
227860: 37 9 61 6 92 1 4
6571872443: 543 121 9 148 2 443
741867: 5 21 9 81 7
177518571: 8 53 611 1 6 17 862 5 3
3596: 44 299 7 78 79
360138: 9 4 1 665 1 2 497 42 3 9
42509: 23 495 82 6 27
28260834920: 785 36 834 79 7 5 118
263067760: 515 714 40 65 85 6 70
256447296093: 860 200 7 35 864 4 9 2
246840600711: 53 3 735 350 92 3 712
13192021941: 409 7 68 56 9 388 5
15576: 7 40 7 3 3 378 8 390 4
2189253: 46 8 73 4 53
1816444120: 605 4 6 3 6 3 4 2 4 1 2 10
46744143989399: 4 6 74 414 3 98 7 2 399
1255715008872: 9 872 2 3 5 8 69 7 8 87 2
118404: 885 12 941 279 4
404222: 1 9 5 389 2 21
154879: 1 580 72 79 39 3
3359259415: 646 26 29 7 2 15
4538379: 3 8 529 233 8 4 513
915: 4 1 640 5 265
581101: 978 3 2 99 8 161
83836120: 20 94 50 94 41 2 72 8
2033775: 38 539 61 99 9 9
692410: 692 3 60 48 5
194576: 304 4 8 8 2
620550: 2 5 5 8 64 9 3 562 9 7
7624704956: 3 93 64 264 8 96 958
964319157: 147 3 78 35 74 84
627487938: 8 2 7 641 7 938
162785716: 37 6 6 3 7 6 3 9 1 4
9445586101: 6 8 8 3 35 45 7 6 6 4 102
539240: 9 21 9 7 65
389898820815: 8 5 538 4 6 3 4 3 601 8 7
10363: 42 856 6 132 1
1039848290: 6 3 26 781 102 1 11
140532: 193 151 13 8 8 574 6
5068: 15 1 7 44 25 423
15985935: 81 8 3 1 9 665
1041709883: 4 795 8 7 1 7 2 2 9 6 646
1432651301: 102 194 724 99 4
4878: 7 114 523 99 5 6 390
4181369637: 4 79 43 880 203
31575685: 4 6 304 7 3 9 3 2 965 8 5
3622: 3 61 3 1 9
982: 99 93 791
27390952984: 22 67 5 5 7 944 980 6 4
184342: 71 88 25 329 14
3392818: 467 24 691 7
10248050597: 2 7 1 5 1 45 2 66 724 9 5
53172521: 870 10 4 61 781 19
11304: 98 3 4 80 9
104258902: 7 7 4 463 7 484 5 82 2 2
5613: 6 19 1 22 117
1148356567: 7 8 274 662 2 745 6 6 6
4084: 5 67 16 7 3
124533: 9 6 5 59 14 2 3 5 8 3 3
2057435762: 74 7 94 74 7 571
6509263: 62 563 280 871 55 37
264982: 325 1 8 42 7 6 2 10
50343: 2 2 44 52 5
28924499: 59 49 84 60 98
8914562171: 6 1 9 9 6 160 5 16 7 3
3969876: 62 8 8 867 946 63
577332: 774 4 492 70 812
7380856: 38 3 30 6 856
159862230: 2 2 8 2 71 46 3 343 7 5 9
103487100: 2 49 51 41 2 977 483 3
2547882725245: 483 5 1 4 145 8 9 524 3
1387400184215: 13 874 100 184 212
10146850566: 241 7 6 2 77 9 6 10 82 6
471205560: 7 89 164 40 49
304928495: 6 813 7 372 95
5476: 67 4 35 39 4 1 4
437171: 261 58 370 634 345
1544843589: 1 544 4 4 3 7 5 9 4 5 645
1148401434: 3 31 77 3 33 133 9 95
67822: 73 5 19 8 344 4 91 479
3534850086174: 72 243 233 3 8 5 739 7
479471892: 9 97 355 61 9
1858976: 592 4 608 4 386
200176098: 85 894 7 6 439
17579: 9 8 3 58 1 3
36768: 8 3 60 86 4 16 8 6
9500988: 579 1 780 28 21
1156896: 48 241 1 8 8
66078: 83 79 6 2 440 6
37342237806: 957 922 441 14 8 423
2135507316: 3 130 68 703 5 2 7 314
24622848: 3 206 2 2 384
517132446: 5 2 518 7 8 9 7 687 6 4 1
26297215017: 1 5 41 4 658 7 5 386 1 9
11323819355: 2 45 73 460 41 33
877788: 94 7 30 8 666
34923962: 9 588 1 60 5 303 972 1
2060156: 5 93 6 913 3 9 75 731
1010573: 4 897 516 604 5 7 2
1153373: 6 9 416 49 619 370
188237471616: 338 3 5 926 3 1 9 6 2 16
351420096: 77 386 3 7 22 8 563
28154770105: 554 8 50 7 25 2 33 1 55
797941714: 931 9 6 7 7 7 4 952 68
17307: 20 213 7 8 7 7 841
278861076: 31 562 2 85 1 3 4 468
939675870639: 989 5 366 945 638
3355236: 1 1 531 732 1 91 20 3 5
303636711: 94 3 41 427 73
4553356517: 417 79 36 30 85 5 3
119833217456: 149 79 152 18 2 8
8647858: 2 91 55 716 785 8
70791495: 6 9 5 1 3 9 1 5 9 385 89 5
58703112937: 3 7 592 1 4 73 2 949
1333781: 5 22 7 3 321 5 5
55088: 449 376 49 63 26
14704: 1 235 3 5 9 8 2 4 747 4 4
141818221948: 468 6 97 6 9 336 3 170
106725235: 275 66 6 98 323 4
935144558: 90 36 72 243 4 45 57
4783996: 4 760 23 993 1
7179142: 74 97 91 9 6 216
17860470: 337 9 14 312 10 9 530
82330587240: 56 4 5 56 7 2 810 4 4 5 6
7624624: 9 486 900 38 236 38
3117225: 7 7 52 32 8 9 89 723 5 4
4533781031: 70 6 86 9 4 742 933 17
1152: 62 5 5 190 287
202477780855: 5 278 6 251 2 97 265 7
33930789: 22 8 31 9 6 8 3 4 1
46868: 46 366 1 7 4 491
2234965: 9 9 253 10 9 8 8 9 6 76
56725: 725 78 7 2 513
413871: 5 17 25 8 68
4411739: 612 901 54 8 7 4
280019599: 62 497 5 4 1 3 9 80 599
139832: 4 9 965 2 343 7 7 79 2
6626869: 599 3 86 64 28 2
22108023: 854 2 9 258 5
149940: 80 69 462 480 1
2333364: 52 223 804 4 44
564496918: 5 748 658 4 9 6 918
7310688: 4 7 1 82 40 4 9 51 21 92
232322688: 6 74 64 9 24 84 32
472649: 2 28 8 20 45 606
4985: 980 2 13 2 5
173748825: 9 48 75 97 419
4032: 53 90 75 6 18
2819352: 73 3 16 711 77 6 19 37
956118: 557 3 5 570 786 8 4
16924517: 2 7 4 67 6 21 984 9 701
92: 9 4 1 57 1
812046062: 7 2 5 4 32 38 96 7 9 581
87867529187: 67 2 7 4 906 6 8 3 8 3 87
28749775: 2 4 2 61 5 82 32 11 49 5
6310651: 983 3 8 8 2 51
5014704: 4 2 747 839 753 2 7 78
15436752: 47 207 327 27 33 3
83562106762: 83 8 9 498 383 2 6 764
60877371: 37 5 688 42 41 17
310856: 677 5 25 6 91
116678676: 1 29 177 4 511
4599: 434 6 88 87 78
1737334: 159 43 4 7 53 657 2 22
512120: 441 3 719 15 9 836 7 4
18168384: 1 7 7 243 971
5436: 424 40 9 734 56
2853540: 30 948 95 39 1
207065: 414 13 5
1387710382: 14 11 9 2 8 6 2 77 9 79
95854: 93 51 3 5 6 8 216 4 4
11476: 98 2 4 110 36
88098: 89 790 1 91 6 3
4650: 77 6 1 22 7
320716: 6 82 41 234 61 82 670
912: 86 7 5 2 41 48 627
164961: 3 674 966 30 92 538 1
138616: 4 3 34 4 835 8 7 2 8
1965: 961 2 45
100926540962: 5 2 9 991 990 6 1 960
3194282223: 195 5 426 400 3 9 57
2377: 885 5 902 520 65
875172675: 2 485 3 9 4 8 6 84 3 2 3 5
3527088: 2 2 38 17 89 32 39 1 6
36414372: 9 749 25 4 1 6 917 8 7 2
1956972: 4 1 7 1 4 4 7 1 728 48 6 2
186714996: 891 7 3 3 7 99 8
8779120: 10 381 1 7 23 20
58735380: 97 89 220 10 6
1546053: 81 320 590 52 6 5 87 6
891089745: 9 521 946 38 5 4
18500736: 2 2 5 925 72 8 9
46556774: 9 33 499 77
506616547829: 7 79 2 5 152 9 1 3 6 2 6
9423: 32 2 9 27 1
249538843: 4 9 13 4 23 5 709 3 41
9677312192: 1 23 224 2 6 5 3 2 180 9
367662: 3 9 1 5 2 1 276 37 8 7 7 8
479671395840: 3 4 91 8 9 2 5 812 46 4 7
2408811: 96 305 11 9 340 4 5 6
16956800: 33 8 950 2 32
461607: 45 985 4 87 9 835 2
776: 1 97 8
13140615: 3 73 2 6 5 47 8 5 5 1 1 9
28824: 1 5 46 3 58 1 3 9 87 1
95810721: 1 94 791 1 18 665 58
117868923: 81 1 97 928 3 5
1068: 86 3 7 41 4 53 874
6282: 1 27 21 66 65 9
7620674: 5 71 20 487 94 93 2
7935824: 7 819 739 22 1 5 13 78
199809421: 694 64 75 9 499 335 5
30580: 305 4 3 36
13500687200: 68 3 266 180 572 7 5 2
127633711135: 1 99 7 8 7 4 8 555 2 8 5 3
194559682: 6 648 5 12 1 2 22 5 679
116522137: 82 19 59 44 1 35 34 4
19715: 98 2 9 7 16
101871: 38 991 99
258511840: 5 42 1 54 709 149 32
2391735493: 32 765 3 7 35 458 34
187570353992: 4 9 9 9 7 7 9 3 4 344 7 49
360374400018: 200 208 625 8 36 5 5 8
2230817787: 4 58 2 2 1 181 7 5 9 1 9 3
401494: 3 4 99 1 938 6 4 6
733544: 4 2 4 8 382 2 8 2 7 8 3 5
77590906330: 8 3 4 76 8 2 42 7 4 68 8
2004866958: 250 6 8 66 933 1 25
533045520: 5 6 6 6 6 3 7 521 8 5 7 6
5413: 8 4 451
3651758346: 942 546 71 10 1 45
1125824227545: 3 3 4 1 8 533 87 5 7 8 6
190513132: 6 24 9 49 6 5 9 3 5 5 4 99
934891: 9 7 26 4 2 8 2 8 82 1 90
1137: 8 43 20 86 688
17531: 89 84 10 221
118321967: 8 9 704 65 6 7 4 3 631 2
638843084623: 1 71 3 7 5 4 5 578 179 1
98230: 29 2 2 1 6 2 90 1 7 64 8
13444932444: 8 42 123 216 95 1 4 2
71659034: 626 1 2 9 25 9 95 8 114
32711009: 26 141 51 15 1 88
1684270: 1 280 7 794 92 1 7 59
29916: 1 86 82 339 6 4
4811820531: 4 82 1 3 4 2 2 90 611 8 4
9867762: 10 1 8 3 1 2 9 4 188 3 3 2
510: 457 24 30
3353882283: 3 93 490 2 2 757 3 1 3 8
61: 5 17 9 1 32
1727: 708 898 26 86 8
223611912: 7 7 3 1 451 86 231 9 3 8
116475838: 8 840 24 549
2156733: 7 8 7 2 4 533 8 1 37 730
49787: 4 8 645 883 258
771230164: 9 2 97 4 1 9 808 2 4 6 6
5275: 105 5 5
69778355889: 802 87 4 2 5 9 5 9 99 91
911554728: 4 5 2 3 9 4 236 93 7 8 5 6
704196331803: 704 1 963 31 802
472914541: 28 67 78 4 9 3 7 9 91 7 9
7257683: 93 868 6 817 89
36471370885: 300 770 3 47 6 2 86
33958632312: 19 992 99 99 248 73
2488243: 4 8 74 4 190
26913521521: 7 8 9 6 9 4 49 6 35 1 67 4
52685: 405 98 23 8 8
4406699117: 5 974 4 4 9 2 5 9 91
795088373103: 158 978 39 67 4 6 21 5
305130396: 9 5 1 59 3 347 9 67 12
531: 47 3 7 1 1 9
1192268044384: 3 871 88 35 4 435 4 31
6392322: 7 63 351 7 8 251 44 27
69739288: 8 71 7 4 8 84 4
523483: 87 2 3 5 2 91 2 692
802030: 6 2 81 86 22 30
626316838: 41 201 38 2 419 416
18023805: 969 31 6 60 42
2762443: 15 64 777 45 1 204 4
71441: 87 81 96 9
93838674: 6 7 55 7 155 70 9 97
7306479: 3 2 1 283 9 63 9 7 9 5 8
314885: 9 34 8 8 12 70 6
4845827: 484 5 102 697 31
22788228: 418 9 8 1 68
135358192800: 7 2 3 1 5 4 949 8 5 80 9
1546: 4 51 96 2 1 26 3 909 7
361176914: 3 848 2 49 7 2 2 8 2 914
9544257: 7 7 9 42 3 2 7 5 546 27
526: 65 8 2 4
1932693441: 53 67 7 8 9 4 29 9 780 1
821715: 33 83 3 14
1810654: 22 81 975 556 321
415368: 31 3 735 2 54
2650: 39 79 21 295 96 5
3277842795: 8 69 22 4 6 2 5 5 3 613
787763809: 692 4 8 363 53 6 35 49
140345412505: 296 4 947 24 43 58 5
397671263: 9 1 3 1 57 6 8 5 4 3 126 3
2875941: 4 603 5 44 47 8
4621566793294: 8 932 452 5 22 61 853
8968000: 467 8 59 32 3
6027428: 5 675 352 423 6
1724: 8 2 12 694 46
392769: 9 6 907 5 1 1 39 5 3 3 9
3925: 759 4 1 61 827
1486822: 82 594 5 9 17 3 2
2400: 6 7 143 90 8
4286544: 347 9 807 1 546
3307871: 3 4 4 4 6 6 72 2 285 23 7
398278260: 3 7 81 6 9 7 526
59250: 6 93 623 28 79
122906054: 30 884 64 3 92 7 53
32646: 3 1 627 9
2980467454: 8 8 9 9 3 3 4 46 7 440 5 9
55943811099: 530 535 758 770 2 9 9
2168287875: 6 86 257 2 42 7 5
481195320: 789 790 772
62339349: 4 1 8 930 8 9 3 16 9 3 2 4
7157155545: 3 15 3 973 5 4 3 5 55 48
210528982: 4 9 9 32 521 57 866 52
1545266035: 61 62 6 32 5 680 9 24
51739: 9 5 2 7 82 79
6102859: 60 181 756 90 8 3 76
10159693192: 4 27 33 114 721
214575225: 8 51 6 4 9 837 675 9 5 9
7028944: 47 7 4 3 9 7 1 51 2 376
23322867: 9 49 29 28 4 574 7
902118625817: 883 8 759 94 625 819
713256123099: 742 155 47 961 9 1 97
1755816105888: 698 3 776 796 351 3
396663497: 1 983 2 634 97
1348244080: 65 768 205 78 1
462274: 248 8 233
61266: 8 7 87 6 69
5882741765: 1 7 3 5 3 1 427 2 4 8 8 8
766181250: 485 29 811 771 15 50
1716743769: 58 8 701 754 7 308 69
21777: 2 9 3 5 65
1553336: 56 295 94 1 393 59
40332646896: 873 35 22 781 6 6
4110: 35 5 80 156 754
37564007116818: 93 910 8 5 7 1 116 818
26127012: 9 24 1 7 4 98 4 2 7 3 33 9
12548006988763: 979 1 1 4 2 86 149 763
636520546: 998 1 4 7 7 425 4 77 85
14145768: 829 4 7 4 651 4 889
8725165: 9 173 5 903 6 8 3 65
474882020: 448 53 101 5 4
124342128: 983 5 871 614 28 35 8
358665062: 5 3 80 8 1 1 8 8 337 3 6
669561657: 3 28 2 607 2 3 6 1 656
117632: 8 3 208 555 4
1220055: 217 56 7 40 39 119
205348861: 9 56 3 716 61
8410710: 5 64 4 8 98 7 363
7928792: 376 46 8 80 3 4 3 3 764
5799083120393: 3 5 6 47 4 7 170 7 7 83 5
107463712: 9 6 1 6 60 9 5 9 9 2 8 67
10207: 91 1 33 428 549
43330935: 8 9 53 7 68 947 82 3
19476019: 1 9 3 1 72 94 3 7 6 4 34
284580: 15 8 1 20 9
2144840841: 57 38 505 9 99
22349258705: 19 1 26 5 9 1 732 8 5 8
487325080187: 39 935 650 160 38 5
24929138: 1 395 91 18 932
65799405: 5 3 1 263 435
2216: 82 4 6 24 8
980777690: 954 220 584 835 50
561801738: 1 4 50 8 15 4 325 673 8
1490988: 93 2 8 2 987
7942059112: 7 9 1 1 3 8 5 5 4 582 9 12
4180933171: 830 6 5 93 260 54 3 27
330636: 715 387 13 3
1069672872524: 1 298 9 795 7 5 2 252 4
5186234: 5 1 794 18 50 37
5016205: 722 43 6 426 1 4 7 1 56
258833147: 4 8 7 6 548 513 23
28645434967: 203 1 61 14 25 9 967
9181353: 69 98 751 1 351
4124314800: 66 7 8 687 772 190 9 6
291900470: 6 7 3 6 2 525 2 2 61 9
4665835983: 170 296 58 35 983
30415786635: 713 864 927 394 108
346694957: 385 21 561 42 9 531
76864: 70 6 864
127218604: 18 7 8 3 96 14 8 5 2 2 6 2
98043728: 4 3 79 5 1 8 19 8 6 9 2 51
1125553: 29 83 55 52 3
142859: 9 51 150 3 205
8996988138: 493 389 943 8 65 5 4 7
32707052: 6 51 5 5 8 4 6 218 3 3 5 2
4186878922: 41 7 74 94 73 5 902 20
50607732: 3 3 132 3 57 9 113 43 3
1318255: 12 186 8 59 905
320484960: 6 1 707 6 1 914 487
2068225: 3 8 9 44 227
79076: 85 3 310 26
13952270: 29 979 64 137 2 95
106877774: 2 490 37 93 6 113 567
3846: 3 1 8 37 1 7 21 7 73 149
13619850960: 25 28 444 860 673
2074682368: 3 226 8 4 226 2 5 8 5 65
7226571570: 722 182 91 384 1 567
900282: 89 9 217 988 77
1127715: 6 97 5 787 4 76 816 3
16058952397: 35 608 392 741 394
26843: 886 8 4 3 3
35145018: 1 66 39 290 50 19
1278: 93 534 2 15 9
24038: 893 521 17
196530656: 4 1 721 784 3 956 8 7
47772498: 4 7 5 3 748 6 8 7 15 4 9 9
8610: 70 4 11 2 795 69
423060737457: 4 8 7 2 3 54 3 2 1 886 9 3
63375360: 75 109 77 16 96 5
128980522089: 8 475 3 2 8 418 2 829 2
7945: 5 5 6 843 8
8684275286: 1 784 83 421 6 52 88
3714: 33 4 11
110467308: 5 835 86 819 18 25 84
297205: 7 67 102 342 4
61361: 2 3 2 7 2 361
22550147294: 3 186 4 7 46 5 36 823 4
1052927614392: 2 1 5 564 4 3 3 718 393
3828: 3 58 417 8 7
9729036288: 656 4 16 314 82 9
5831042160: 915 619 3 834 4 5 514
10047756640: 8 301 4 2 7 6 8 11 8 388
34334730: 51 440 1 90 17
2355371884: 7 7 8 1 5 3 2 3 7 184 4 4
7720: 5 69 7 4 433 1 9 8
508919: 92 204 79 1 3 2 9 5
2749463626: 642 921 5 465
148277603765: 1 5 43 40 7 4 39 8 429 5
6912: 81 51 84 2 55 89 12
2519958: 7 304 32 8 253 3 9 66
68495: 42 634 7 38 95
37935797473371: 76 176 3 498 73 368
4088514: 20 460 1 85 14
42340486560: 727 7 104 4 2 1 6 5 57
93889045874: 7 7 3 3 722 4 7 9 8 5 873
111628894: 29 375 84 38 702
1836669: 5 4 34 602 68
127205: 9 10 8 4 3 893 8 3 5 8 1 5
10169671: 1 72 7 6 997 8 1 46 4 3 2
237605921214087: 62 52 7 874 38 140 87
37392: 1 5 276 175 82
33961623: 1 6 1 6 99 3 61 543 6 71
37304: 8 4 5 620 65 2 38
328923190: 876 126 113 7 4 415
9004522: 852 15 2 21 352 8 2
6277: 6 274 4
799: 18 5 3 7 5 3 4 8 4 7 7 305
815: 5 567 75 167 1
1907156: 7 55 29 4 2 4 3 733 5 6
54232722: 8 11 1 1 12 3 3 6 51 438
90356: 6 53 204 65 98
80393954535: 6 980 8 699 3 9 815
585333000: 81 1 6 8 557 1 4 7 6 3 7 3
1740698240: 1 89 16 4 1 8 19 8 944 5
27080469199: 147 5 997 177 196
397655: 4 80 674 4 54
32581186: 58 546 913 14 4 3
1411288: 329 6 788 5 1 50 5 7 5 3
686822508: 6 837 3 29 455 10
14329614: 37 387 10 540 74
900648672: 172 596 9 8 44 89 37
12010046: 6 8 72 100 47
51520200: 831 9 19 27 613 1
44168970: 72 20 6 8 1 2 77 67
53032: 529 796 8 1 4
88279: 5 38 7 293 86
37669257997338: 570 7 463 33 292 1 66
37263336: 423 4 47 22 4
1953126: 196 1 2 2 4 411 3 7 8 3
18262: 25 181 88 80 55
745577968: 9 31 27 70 2 4 5 8 7
325718593204: 3 1 6 3 7 1 8 3 9 35 132 2
70183: 7 5 92 597 83
292206: 8 528 2 973 5 31
35684388: 9 6 531 4 7 9 1 5 7 8 798
4518001568: 8 4 4 3 3 9 900 4 1 563 6
353907: 69 907 3 870 5 6
14174170: 141 735 60 582 28
4579: 89 80 9 5 3
905743293: 17 4 3 57 9 922 48
4721355: 3 4 6 13 6 8 7 2 8 15 853
28300777: 6 2 7 4 4 3 9 94 4 38 97 8
3059337317: 305 44 49 373 17
123832907: 800 5 7 762 8 412 5 2 6
189634: 9 9 9 624 7
53083923: 906 59 13 852 58 1
117861384016: 5 46 14 5 2 6 8 8 5 5 8 17
1670110: 25 95 542 6 25 54 53
18266750880: 8 28 98 20 86 69 478
846: 754 3 7 6 74
91230: 565 325 22 30
21602109: 878 1 82 6 3 833
383: 6 1 2 178 2 11
4640341196: 5 58 4 4 3 3 99 5 4 3 96
295: 2 7 35 7 5 15
25640: 60 7 42 146
2324: 7 2 7 4 852
2106855: 1 918 527 243 5 6 15
1096879615: 9 1 38 52 20 61 792
206838115: 2 7 2 7 42 7 94 1 7 9 2 5
892: 838 53 1
37042143372: 1 38 45 81 549 26 386
13608260: 9 7 9 3 63 260
10296568836: 260 11 15 801 6 6
1152012: 2 7 410 450 12
15670: 9 174 9
378879: 428 87 522 6 530 762
63217413002: 7 7 10 61 7 1 1 5 3 350 3
214357346: 9 9 2 3 3 9 527 5 9 5 2 6
6384: 1 5 860 5 42 7
3660322: 2 687 53 8 8 1 770 9 20
597992800: 59 79 878 49 1 8 8
2086695: 3 1 8 2 164 1 6 1 3 195 3
56275146474: 7 1 82 572 903 88 71
5878173: 9 97 2 615 9
256125: 69 157 24 3 57 13 112
63613676031: 345 710 62 53 49 17
6041746627: 6 65 6 3 180 1 6 77 25 5
445082: 50 9 4 7 75 92 141 9 5
3336568969: 11 93 3 1 605 537 7 9
14476045838: 720 220 154 45 8 3 8
25791150: 73 65 2 37 350
1770: 4 18 8 2 7
634521698: 96 816 81 89 9
766610: 7 35 29 2 609
165288: 99 8 89 88 6 97
122461717: 2 657 224 269 8 37 5
49274709: 8 61 4 71 709
156370725697: 1 771 2 9 3 3 2 75 5 6 9 6
135450075: 306 97 8 1 42 10 73
178096158: 7 471 686 391 54
37098919: 1 2 6 539 2 1 7 45 1 8 2 2
564755082: 829 68 103 508 2
15091: 84 7 590 978 7
5925156: 4 80 5 6 7 5 7 443 95 8
227132981304: 639 235 47 7 9 1 9 2 3 4
83734: 4 2 3 7 36
4764348248: 231 851 16 64 642 44
201359438581: 33 9 429 677 5 580 1
5382: 32 497 1 9 3
89850: 46 810 70 866 5 50
7095185620: 2 18 2 83 9 388 9 1 34
898628: 89 340 93 429 8
4521335: 2 988 456 2 691 4
825553: 527 522 3 216 56
1437: 6 7 492 737 196
6349783608: 2 98 268 522 216 63
26307: 47 42 204 4 36 79
2664689: 4 37 46 3 295 96 88
517476672: 699 24 5 28 74
906139989: 15 59 62 559 98 8 3
101621401: 3 4 99 29 42 6 841 7
174668041: 291 11 339 3 3 2 1
499516611066: 5 3 6 3 6 238 6 7 55 1 6 3
656734: 5 789 741 21 7 9 34
2964812: 3 4 7 2 36 1 11 813
457793: 526 17 130 8 791 39 5
324783326229: 32 474 9 34 32 62 29
714670: 42 17 552 60 9 46
174857716: 6 9 146 8 734 1 9 18 2 9
22400846: 350 64 843
9498234307: 4 12 966 709 23
16680905: 9 9 156 7 3 17 6 6 4 8 4
405: 9 9 5
4565438: 913 1 5 4 37
214448709: 670 13 423 7 318
2056320: 6 854 62 7 247 378
28709528: 3 580 8 550 57 75 9 8
12101046387: 5 1 86 7 4 3 7 3 638 5 2
9550: 52 7 992 7 58
8347014: 7 32 5 92 9 9 331 443
27345853945: 9 202 2 3 8 3 3 8 4 649 3
154494235766: 772 469 2 1 777 6 5 3 2
55637839247: 6 8 2 44 955 8 7 5 9 93 4
3395211904: 285 9 77 53 415 224
410136: 51 26 7 8
22526624126: 2 555 82 778 854 6
2985869706: 46 9 70 8 5 7 8 7 6 90 9
3423438677: 714 705 958 346 5
784310979: 784 310 89 8 9
2859878751: 3 96 3 36 48 8 37 9 305
900: 38 7 1 4 5
30425338: 30 4 25 33 8
567294: 4 702 3 12 8
33541379: 110 571 6 89 64 775
494363134: 555 4 620 12 89 886
71527: 3 6 5 3 8 6 1 7 12 5 8 7
46980046: 53 721 96 540 43
37206236963: 7 7 88 94 6 9 83 1 61 5 3
50976954: 5 2 2 80 45 37 11 87 14
5203315048: 1 63 2 74 61 27 3 5 50
1939803185: 969 901 59 2 6
1028599911631: 118 20 2 97 1 749 87 1
91641646160: 9 424 43 62 472 206
4808181291: 2 978 9 889 26 12 93
357338008: 706 3 504 8 3 2 9 6 789
81364559: 800 13 56 8 556
1517735009: 1 6 414 291 923 787
10942: 2 2 130 72
2927: 74 5 3 8 1 31 9 646
21363940: 72 22 73 3 49 829 260
1075962223: 21 512 7 615 53 670
91216741: 912 1 30 37 41
3020021513: 749 56 686 8 458 9 2
3412491525: 28 57 29 4 8 912 3 23
586986542: 34 36 81 832 2 7 8 7 68
3985200: 3 3 3 616 9 118 4 9 45
25153680: 82 71 3 1 60 8 3 39 24
5121: 557 3 8 1 9
4361404380: 9 72 779 4 9 6 6 2 5 12
4633574: 1 8 52 2 26 4 9 4 4 7 6 41
2500: 10 3 62 200 7 433
205765071: 237 599 5 866 4
475225461: 938 7 336 6 687 90 81
67637: 67 633 6
62118081357: 1 2 9 1 90 290 4 5 2 4 90
3666: 3 58 7 8 8
149245: 4 382 4 19 718 5 87 28
561006667: 82 5 68 66 6 6
83570460265375: 92 6 318 188 547 5 85
71579598: 29 75 67 329 55
108468786: 27 117 4 747 42
108885688319: 1 3 7 7 770 5 4 2 88 320
274800836: 5 60 2 6 49 4 47 435 7 9
161022283: 38 7 604 358 232 51
19692: 41 990 933 9 43
1814645: 9 270 89 7 85 695
5522958: 9 3 9 6 1 63 769
14581981: 11 9 6 9 6 8 2 7 4 60 63
150150: 5 4 90 3 69 5 18 6 72
34959356: 368 3 678 22 14 6 1 1 3
2325598: 48 57 850
13630671514: 1 45 3 4 9 526 31 171 3
83325934080: 8 7 8 9 7 182 7 6 32 9 1
37047437: 4 583 9 777 2 1 9 10 4 1
2664496: 98 65 9 7 9 3 709
2768330498: 1 893 31 30 498
865227: 50 2 57 167 33
5902050651: 843 1 487 1 7 7 1 48 54
7499664484: 832 694 602 9 4 84
7952436: 770 1 2 233 6 1 1 436
8294452: 2 9 3 8 66 5 1 5 41 8 7 2
6456627216: 4 3 9 26 4 86 3 8 544 1 8
2802: 255 2 169 9 4 48 2
699018: 2 7 2 6 6 353 7 5 4 39
2189890971124: 92 980 9 785 704 2 3 1
33192972: 646 99 519 826 20
6335442: 2 6 564 13 3 4 1 2 96 6
8535690: 1 866 1 2 9 6 2 9 5 9 3 90
5936091: 89 900 3 49 6
1429232: 79 7 8 626 9 500 112 7
12011121824: 15 8 11 12 1 824
    "
}
