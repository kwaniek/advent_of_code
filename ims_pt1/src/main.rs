use std::collections::HashMap;

fn input() -> Vec<&'static str> {
    vec![
        "ivyhczwokexltwhsfamqprbnuy",
        "ivjhcjdokexltwwsfamqpabnuy",
        "ivjhczdokebltwgsfydqprbnuy",
        "ivjhcadokexftogsfamqprbnuy",
        "idjhczdokexltwgsfacqprbnuh",
        "ivjhgzookexltwgsfamqjrbnuy",
        "uvjhctdokexltwosfamqprbnuy",
        "ivrhczdokexltwhzfamqprbnuy",
        "ivjhczuxkexltwgsfamqprbney",
        "ivjhczdokemltwgsfadnprbnuy",
        "ifjhczdokexltwgsfamqprbkuf",
        "ivjhkzdokltltwgsfamqprbnuy",
        "ivjuczdhkexltwgsfamqprtnuy",
        "ivjhjzdxkexltwgsfapqprbnuy",
        "ivlhczdoxexltwgsfamqprgnuy",
        "ivjhczdoknxltwgssamqsrbnuy",
        "ivjhczdokexltwgswadqprbruy",
        "ivjhczdokexthwgsfampprbnuy",
        "uvjhczrozexltwgsfamqprbnuy",
        "ivolczdokexltwgsffmqprbnuy",
        "ivjhczibkexltwgsfamoprbnuy",
        "ivjhczdokefltmgsfamqprbouy",
        "ivjhczdobexltngsfamsprbnuy",
        "ivjhczdokexltwvsfxqqprbnuy",
        "dvjhczdokexucwgsfamqprbnuy",
        "kvjhszkokexltwgsfamqprbnuy",
        "ivjhczdokexrtegsfamqprbnus",
        "ivjhctdokexltwglgamqprbnuy",
        "ivjhczdozexutwgsfamqdrbnuy",
        "ivjhczqokemltwgsfakqprbnuy",
        "uvjhczdokexlqwgsfadqprbnuy",
        "ivjhczdohexltwglffmqprbnuy",
        "izjhczdokexltwgsfamqprbsqy",
        "iajhczdokwxltwgjfamqprbnuy",
        "ivjfczdokexllwgslamqprbnuy",
        "ivjhczdoyexltwgsfamqdrbnxy",
        "ivjhczdokekwtwssfamqprbnuy",
        "ivjhcodokexltwgsfamqirxnuy",
        "ihjhhzdokexltwgsfamqlrbnuy",
        "ivjdpzdokexltwfsfamqprbnuy",
        "ivjhcpdokexltwgsfamqqrbruy",
        "qvjhcziokexltwgsfamqprbnny",
        "ivohczdomexltwgsfkmqprbnuy",
        "ivjhczhokhxlywgsfamqprbnuy",
        "ivjhczdokexltwgmffmqprbruy",
        "ivjhczdokqxltwgcfamqprbnyy",
        "ivjhczdokepltwgsfamcprbnay",
        "ivjhczdokexltwgseamqpmbnua",
        "ivjzczdokexltwgszamqplbnuy",
        "ivjhczpokexltwgvfgmqprbnuy",
        "idjhczdokexltwgsxamqprbndy",
        "ivjhczdxkexltwgcgamqprbnuy",
        "ivjhczdckexatpgsfamqprbnuy",
        "ivjrczdorexltwgsfamqprbnvy",
        "ivjoczdokexltwgswamqprbtuy",
        "iylhczdokexltwgsfamqpxbnuy",
        "imxhczdokkxltwgsfamqprbnuy",
        "ivvhczdoktxltwgsfamaprbnuy",
        "ivyhczdokexltwhsfayqprbnuy",
        "ivjhcrdokexltegsfamqprbnum",
        "rvjhezdokexltwgsoamqprbnuy",
        "ivjzczdokexlbwgsfkmqprbnuy",
        "ivjhczdokelltwgsyamqprbnoy",
        "ixjhczdorexltwgsfamqprbuuy",
        "ivjhczpokexdtwglfamqprbnuy",
        "ivjhczdokexltwgfgamcprbnuy",
        "ikjhczdokexltwgsfamqirbnux",
        "ivjhczdopjxltwgsfamqprbnny",
        "ivchczdokexltwgniamqprbnuy",
        "ivjhczdooeqltwgsfamqprbniy",
        "ivjhcldonexltwgbfamqprbnuy",
        "ixjhczdokehltwgsfamqprbnuf",
        "ivjhczdckefltwgsfamqppbnuy",
        "ivjhczdoqrxltwgsfamqprbnun",
        "ivjhczdokcxltwgmfarqprbnuy",
        "ivjhcziorexltqgsfamqprbnuy",
        "ivjhwzdokexltwgnfamqprbcuy",
        "ivjhczdoqexltwgsfazqprunuy",
        "iijhczdokexltwgsyamqprbnug",
        "ivjhczdokexltwgxfamhprbnry",
        "ivjhczdakexltwgsfaeqlrbnuy",
        "ivjhqzdokehltwgsfampprbnuy",
        "ivjhczdokexltwlsfpmyprbnuy",
        "ivjhfzdoktxltwgsfamzprbnuy",
        "ivlhvzdokexltwgsvamqprbnuy",
        "ivjhczdbkexltwgsaamqprfnuy",
        "ivahcedokexltigsfamqprbnuy",
        "cvjhczdokexltwgsfamapibnuy",
        "ivjhczkokbxltwgsfbmqprbnuy",
        "pvjuczdnkexltwgsfamqprbnuy",
        "iyjhczdckexotwgsfamqprbnuy",
        "ivjhzzdokvxltwgsfamqprbnuo",
        "ivjhczdobexltwgsxamqprbnry",
        "ivjhczdokexltwgsfaprprbnub",
        "ivjhczdokexltwgofarqprbkuy",
        "ivjhczdokexltwgbfymqprbnhy",
        "ibjhczdokexltwgsfkmqpvbnuy",
        "ivjhczdzkexlywgsfacqprbnuy",
        "hvdhczdokexltwglfamqprbnuy",
        "ivjhczdokexrtwgsfamqprbsuh",
        "ivjhczhokexltngsfamqpjbnuy",
        "ivjhcsjokexltwgsfaeqprbnuy",
        "ivjmczdokexltmgsfamqpbbnuy",
        "wvjhczdokexltwgsfamkpkbnuy",
        "icjhpzdoaexltwgsfamqprbnuy",
        "ivjmczdhkexltwgsfzmqprbnuy",
        "ivjhczdokexytwgsfamqprbwug",
        "ikjhczdjkexljwgsfamqprbnuy",
        "ivjvcdmokexltwgsfamqprbnuy",
        "ivjhazdorixltwgsfamqprbnuy",
        "ivchczdokexltwgsfamzprenuy",
        "ivjcczdokexlttgsfamqpmbnuy",
        "ibchgzdokexltwgsfamqprbnuy",
        "ivjhczdokepltwgsfamqpeenuy",
        "ivjnwzdokexlrwgsfamqprbnuy",
        "ivjhczdokexitwgsfadqlrbnuy",
        "icjhcrdokexltwgsfamqkrbnuy",
        "ivngczdokexltwgsfamqprbyuy",
        "ivjhuudokexlvwgsfamqprbnuy",
        "ivjhczdnkexltwgsfhmqpxbnuy",
        "itjhczdokexltwvsfamgprbnuy",
        "ivjhcddokexltwgsfakqprbnny",
        "ivjhuzdojexltwfsfamqprbnuy",
        "idjhczdokexltwgsfamqukbnuy",
        "ivjhczdokexlzigsfamqprbngy",
        "ivjwczdokexltwgufamqprbnuo",
        "iijhczdokexltwfsfadqprbnuy",
        "ivjhczdukexdtwgsfamqpsbnuy",
        "idjhczdokexllwgssamqprbnuy",
        "zvjhczdokexrtwgsfamqplbnuy",
        "ivphczdofexltwgefamqprbnuy",
        "ivhhczdokexlpwgsjamqprbnuy",
        "ivjhczdovexltwgsfamqprhnuj",
        "ivjhczdoklxltwgseamqprlnuy",
        "ivjhcqdokexltngsfamqprdnuy",
        "ivjhczdoifxltagsfamqprbnuy",
        "izjhczdokexltwjsramqprbnuy",
        "psjhczdokexlgwgsfamqprbnuy",
        "ivjhcadokexltwgsfsmqwrbnuy",
        "ivjhczdokexltwgsfawqiibnuy",
        "ivjhczkokexhtwgsfamqprbnuk",
        "ivjhcmdukexltwgsfamvprbnuy",
        "ivjlczdokexltwgsfamquibnuy",
        "ivjhczdokexntwgyfamqprbniy",
        "ivjhczdokexltwlsfafqprbnuc",
        "ivjhczdosexltrtsfamqprbnuy",
        "ivjhcznokexbtwgsfafqprbnuy",
        "ivwtczdotexltwgsfamqprbnuy",
        "ivjhvzdokexltigsoamqprbnuy",
        "ivjhcmdokexltwasfamqirbnuy",
        "ivthczdokexltwgsfaydprbnuy",
        "ivjhwzdskexltwgsfamqprbnus",
        "icjhczdosuxltwgsfamqprbnuy",
        "ivjhczdokexltwgstamqbrmnuy",
        "iejhczuoktxltwgsfamqprbnuy",
        "ivjhczdokeqltwgskamqprbniy",
        "ivjhlzdokexltugsfamqprbpuy",
        "iwjqczdckexltwgsfamqprbnuy",
        "ivjhwzdokexluwgsfxmqprbnuy",
        "ivjhczdokexltwgwfwmqprbguy",
        "gvjhczkokexltwgsfgmqprbnuy",
        "ivjhczdoyexlhwgsfamqprbnoy",
        "cvjhczdokexltwgsfomqprinuy",
        "vvmhczdokexltwgsfamqprbnun",
        "vvjhczdokexltwgsftmfprbnuy",
        "ivkhckdokhxltwgsfamqprbnuy",
        "iyjhczdkkexltjgsfamqprbnuy",
        "ivlhczdokexltwgsfamqyrbhuy",
        "tvjhmzdokexltwgsfamqorbnuy",
        "ivjhczdokexltwvsfamqprbnxi",
        "ivjhczdowexltwgswamqerbnuy",
        "wvjiczdomexltwgsfamqprbnuy",
        "ivjpizdokexltwgvfamqprbnuy",
        "ivjhuzdokexlzwgspamqprbnuy",
        "ivjhczdokeyltwgkfamqprdnuy",
        "jvjhczdokexlnwgsfamqirbnuy",
        "ivjheidokexltwvsfamqprbnuy",
        "mvjhczdokexltwgsfamqyrsnuy",
        "ivjhazdykexltwgsramqprbnuy",
        "ivjkcodokexltwgsxamqprbnuy",
        "ikjhczdoktxltwgpfamqprbnuy",
        "ivjhyzdfkexmtwgsfamqprbnuy",
        "ivohczdokexltugsfamqprynuy",
        "ivjkczdqkexltwgshamqprbnuy",
        "ivjhczdokexltwgskamqynbnuy",
        "icjhczdokexltwgofamrprbnuy",
        "ivjhlzdokealtwgsfamqsrbnuy",
        "ivehczdybexltwgsfamqprbnuy",
        "ovjhczdokexltwgsfamqirbnuo",
        "ivjoczdokexltwgsfamqurbnty",
        "ivjmczdokexltwgsfrmqprnnuy",
        "ivjhczdowpxltwgbfamqprbnuy",
        "ivjhczdokexltwfsfamqkrgnuy",
        "ivjhwzdokexltwgsfavqprbnuq",
        "jvjhczdokexltwgsiamqprbnny",
        "ivjhlzdouexltwfsfamqprbnuy",
        "ivjhczdokexltwgsfamqbrbnlv",
        "iwjhczdokexltwgsfapqprbnqy",
        "idjhczdokexltwgsaamqrrbnuy",
        "ivjhjzdopepltwgsfamqprbnuy",
        "ivjmczdokejltwgsfamqpbbnuy",
        "ivjhczdokexltwgsuamdprvnuy",
        "injhczdokexltwgefamqurbnuy",
        "iujhczdokexltwgsaamqjrbnuy",
        "ivjhczdokexltwgvfaaqprbnly",
        "ivehczdokexltwgsfamqppbnui",
        "ivxhczdodexltwgsfamqplbnuy",
        "ivjhczfokexltwgsfamqpwbauy",
        "ivjhcztwkexhtwgsfamqprbnuy",
        "ivjeczdokexltygsfmmqprbnuy",
        "ivjhchdokexltwgsmameprbnuy",
        "ivkhczdoklxltwggfamqprbnuy",
        "ivjhczdzkexltwhsfamqprjnuy",
        "ivjhcedokeultngsfamqprbnuy",
        "ivjhczdokexvtwgseabqprbnuy",
        "ivjhczdooexltlgsfamqpibnuy",
        "ivjgczvosexltwgsfamqprbnuy",
        "ivlhczwokexltwgsfamqmrbnuy",
        "lvjhczdokexutwgsfamrprbnuy",
        "ivahczdokexpdwgsfamqprbnuy",
        "ivjhcznokexltwhsfamqpnbnuy",
        "ivjhczdpkyxltwgbfamqprbnuy",
        "ivjhnzdokexltwgsftmqprinuy",
        "ivihczdokexltnhsfamqprbnuy",
        "ivjhcbdokevltwgsfamqprbauy",
        "hgjoczdokexltwgsfamqprbnuy",
        "dvjhczdckexltwgsfamqpybnuy",
        "ivjhcadokesltwgsfsmqwrbnuy",
        "ivjhwzdokexlttgsfamqprbney",
        "ivjhcidokexltwgofamqfrbnuy",
        "ivokwzdokexltwgsfamqprbnuy",
        "ivjiczdokexltwgsfaqqarbnuy",
        "ivjhczdokexqtwfsfamgprbnuy",
        "ivjhczdokealtwgsfamqerbnqy",
        "ivjhczdskexltwgsfamqprznuu",
        "ivjhwzdokexltwjsfdmqprbnuy",
        "ivjhczaokexlzwgsfamqprbnus",
        "ivjhczdokexltwosfamqnrbnux",
        "ivjhczdokexlqwgsfamwprcnuy",
        "ivjhczdqkexltwgswamqpcbnuy",
        "ijjhczdokexnttgsfamqprbnuy",
        "ivjhcedckexltwgsfamqprbnpy",
        "ivjhczdokeyltwgsfamqshbnuy",
        "ivjhczdokexltsgsfamqpmznuy",
        "ivjlczdtkeiltwgsfamqprbnuy",
        "ivjhczdokexltwgsfkmtprbnby",
        "ivjhnzdozexltwgsfamqprbnuc",
        "xqjxczdokexltwgsfamqprbnuy",
        "ivjhczdokeyltwgsfamqnrbnuw",
        "ivjwczgokexltwgsfamvprbnuy"
    ]
}

fn the_same_letter_appears_n_times(word: &str, n: i32) -> bool {
    let mut naive_group: HashMap<char, i32> = HashMap::new();
    for letter in word.chars() {
        let new = match &naive_group.get(&letter) {
            None => 1,
            Some(i) => *i + 1
        };
        &naive_group.insert(letter, new);
    }
    naive_group.iter().any(|(_k, &v)| v == n)
}

fn count_two_letters(ids: Vec<&str>) -> usize {
    ids
        .iter()
        .filter(|w| the_same_letter_appears_n_times(w, 2))
        .count()
}

fn count_three_letters(ids: Vec<&str>) -> usize {
    ids
        .iter()
        .filter(|w| the_same_letter_appears_n_times(w, 3))
        .count()
}

fn distance_between_words(word1: &str, word2: &str) -> i32 {
    let mut diff = 0;
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    for i in 0..word1.len() {
        if word1[i] != word2[i] {
            diff += 1;
        }
    }
    diff
}

fn the_same_letters(word1: &str, word2: &str) -> String {
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let mut result: Vec<char> = vec![];
    for i in 0..word1.len() {
        if word1[i] == word2[i] {
            result.push(word1[i])
        }
    }
    result.into_iter().collect()
}

fn main() {
    let checksum = count_two_letters(input()) * count_three_letters(input());
    println!("Checksum {}!", checksum);
    'outer: for word1 in input() {
        for word2 in input().iter().filter(|&it| !it.eq(&word1)) {
            if distance_between_words(word1, word2) == 1 {
                println!("{} / {}", word1, word2);
                println!("{}", the_same_letters(word1, word2));
                break 'outer;
            }
        }
    }
}
