use expect_test::{expect, Expect};

#[test]
fn t() {
    assert!(wn::lmf::from_file("this-file-does-not-exist.xml").is_err());

    let root = wn::lmf::from_file("tests/fixtures/wn-2022.xml").unwrap();

    assert_eq!(root.lexicons.len(), 1);

    let mut lexicon = root.lexicons.into_vec().remove(0);

    assert_eq!(&*lexicon.id, "oewn");
    assert_eq!(&*lexicon.label, "Open English WordNet");
    assert_eq!(&*lexicon.language, "en");
    assert_eq!(&*lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        &*lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!("2021", &*lexicon.version);
    assert_eq!(
        Some("https://github.com/globalwordnet/english-wordnet"),
        lexicon.url.as_deref(),
    );

    assert_eq!(lexicon.lexical_entries.len(), 161221);
    assert_eq!(lexicon.synsets.len(), 120068);

    fn check<T: std::fmt::Debug>(t: T, expect: Expect) {
        expect.assert_debug_eq(&t)
    }

    lexicon.lexical_entries.sort_by(|a, b| a.id.cmp(&b.id));
    lexicon.synsets.sort_by(|a, b| a.id.cmp(&b.id));

    check(
        &lexicon.lexical_entries[..5],
        expect![[r#"
            [
                LexicalEntry {
                    id: "oewn--ap-hood-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'hood",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-hood__1.14.01..",
                            synset_id: "oewn-08242255-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-s_Gravenhage-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'s Gravenhage",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-s_gravenhage__1.15.00..",
                            synset_id: "oewn-08970180-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-tween-r",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'tween",
                        script: None,
                        part_of_speech: Adverb,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-tween__4.02.00..",
                            synset_id: "oewn-00252367-r",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn--ap-tween_decks-r",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "'tween decks",
                        script: None,
                        part_of_speech: Adverb,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn--ap-tween_decks__4.02.00..",
                            synset_id: "oewn-00500491-r",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-.22-caliber-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: ".22-caliber",
                        script: None,
                        part_of_speech: Adjective,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-.22-caliber__3.01.00..",
                            synset_id: "oewn-03157978-a",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target_id: "oewn-caliber__1.07.01..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
            ]
        "#]],
    );

    check(
        &lexicon.lexical_entries[lexicon.lexical_entries.len() - 5..],
        expect![[r#"
            [
                LexicalEntry {
                    id: "oewn-zymolysis-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymolysis",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymolysis__1.22.00..",
                            synset_id: "oewn-13596636-n",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymolytic__3.01.00..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymolytic-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymolytic",
                        script: None,
                        part_of_speech: Adjective,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymolytic__3.01.00..",
                            synset_id: "oewn-03011955-a",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymolysis__1.22.00..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target_id: "oewn-zymosis__1.22.01..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymosis-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymosis",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymosis__1.22.01..",
                            synset_id: "oewn-13596636-n",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymotic__3.01.01..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                        Sense {
                            id: "oewn-zymosis__1.22.00..",
                            synset_id: "oewn-13596429-n",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymotic__3.01.00..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymotic-a",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymotic",
                        script: None,
                        part_of_speech: Adjective,
                        pronunciations: [
                            Pronunciation {
                                variety: Some(
                                    "GB",
                                ),
                                notation: None,
                                phonemic: true,
                                audio: None,
                                value: "zaɪˈmɒtɪk",
                            },
                        ],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymotic__3.01.01..",
                            synset_id: "oewn-03011955-a",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymosis__1.22.01..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target_id: "oewn-zymosis__1.22.01..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                        Sense {
                            id: "oewn-zymotic__3.01.00..",
                            synset_id: "oewn-03011849-a",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-zymosis__1.22.00..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Pertainym,
                                    target_id: "oewn-zymosis__1.22.00..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-zymurgy-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "zymurgy",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [],
                        tags: [],
                    },
                    forms: [],
                    senses: [
                        Sense {
                            id: "oewn-zymurgy__1.09.00..",
                            synset_id: "oewn-06089949-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
            ]
        "#]],
    );

    check(
        lexicon
            .lexical_entries
            .iter()
            .filter(|lexical_entry| lexical_entry.forms.len() > 0)
            .take(3)
            .collect::<Vec<_>>(),
        expect![[r#"
            [
                LexicalEntry {
                    id: "oewn-aardwolf-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "aardwolf",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [
                            Pronunciation {
                                variety: Some(
                                    "GB",
                                ),
                                notation: None,
                                phonemic: true,
                                audio: None,
                                value: "ˈɑːdˌwʊlf",
                            },
                            Pronunciation {
                                variety: Some(
                                    "US",
                                ),
                                notation: None,
                                phonemic: true,
                                audio: None,
                                value: "ˈɑɹd.ˌwʊlf",
                            },
                        ],
                        tags: [],
                    },
                    forms: [
                        Form {
                            id: None,
                            written_form: "aardwolves",
                            script: None,
                            pronunciations: [],
                            tags: [],
                        },
                    ],
                    senses: [
                        Sense {
                            id: "oewn-aardwolf__1.05.00..",
                            synset_id: "oewn-02120828-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-abacus-n",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "abacus",
                        script: None,
                        part_of_speech: Noun,
                        pronunciations: [
                            Pronunciation {
                                variety: None,
                                notation: None,
                                phonemic: true,
                                audio: None,
                                value: "ˈæbəkəs",
                            },
                        ],
                        tags: [],
                    },
                    forms: [
                        Form {
                            id: None,
                            written_form: "abaci",
                            script: None,
                            pronunciations: [],
                            tags: [],
                        },
                    ],
                    senses: [
                        Sense {
                            id: "oewn-abacus__1.06.01..",
                            synset_id: "oewn-02668977-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                        Sense {
                            id: "oewn-abacus__1.06.00..",
                            synset_id: "oewn-02668826-n",
                            status: None,
                            note: None,
                            sense_relations: [],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
                LexicalEntry {
                    id: "oewn-abet-v",
                    status: None,
                    note: None,
                    lemma: Lemma {
                        written_form: "abet",
                        script: None,
                        part_of_speech: Verb,
                        pronunciations: [
                            Pronunciation {
                                variety: None,
                                notation: None,
                                phonemic: true,
                                audio: None,
                                value: "əˈbɛt",
                            },
                        ],
                        tags: [],
                    },
                    forms: [
                        Form {
                            id: None,
                            written_form: "abetted",
                            script: None,
                            pronunciations: [],
                            tags: [],
                        },
                        Form {
                            id: None,
                            written_form: "abetting",
                            script: None,
                            pronunciations: [],
                            tags: [],
                        },
                    ],
                    senses: [
                        Sense {
                            id: "oewn-abet__2.41.00..",
                            synset_id: "oewn-02554908-v",
                            status: None,
                            note: None,
                            sense_relations: [
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-abetment__1.10.00..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-abettal__1.10.00..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-abettor__1.18.00..",
                                    status: None,
                                    note: None,
                                },
                                SenseRelation {
                                    rel_type: Derivation,
                                    target_id: "oewn-abetter__1.18.00..",
                                    status: None,
                                    note: None,
                                },
                            ],
                            examples: [],
                            counts: [],
                        },
                    ],
                    syntactic_behaviours: [],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[..5],
        expect![[r#"
            [
                Synset {
                    id: "oewn-00001740-a",
                    part_of_speech: Adjective,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "(usually followed by `to') having the necessary means or skill or know-how or authority to do something",
                        },
                    ],
                    ili_definition: None,
                    relations: [
                        SynsetRelation {
                            rel_type: Attribute,
                            target_id: "oewn-05207437-n",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Attribute,
                            target_id: "oewn-05624029-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "able to swim",
                        },
                        Example {
                            status: None,
                            note: None,
                            value: "she was able to program her computer",
                        },
                        Example {
                            status: None,
                            note: None,
                            value: "we were at last able to buy a car",
                        },
                        Example {
                            status: None,
                            note: None,
                            value: "able to get a grant for the project",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001740-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "that which is perceived or known or inferred to have its own distinct existence (living or nonliving)",
                        },
                    ],
                    ili_definition: None,
                    relations: [
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-04431553-n",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00001930-n",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00002137-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [],
                },
                Synset {
                    id: "oewn-00001740-r",
                    part_of_speech: Adverb,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "without musical accompaniment",
                        },
                    ],
                    ili_definition: None,
                    relations: [],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "they performed a cappella",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001740-v",
                    part_of_speech: Verb,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "draw air into, and expel out of, the lungs",
                        },
                    ],
                    ili_definition: None,
                    relations: [
                        SynsetRelation {
                            rel_type: Entails,
                            target_id: "oewn-00005041-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Entails,
                            target_id: "oewn-00004227-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target_id: "oewn-00002325-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Similar,
                            target_id: "oewn-00002573-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00002573-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00002724-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00002942-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00003826-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00004032-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00004227-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00005041-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00006697-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00007328-v",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hyponym,
                            target_id: "oewn-00017024-v",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "I can breathe better when the air is clean",
                        },
                        Example {
                            status: None,
                            note: None,
                            value: "The patient is respiring",
                        },
                    ],
                },
                Synset {
                    id: "oewn-00001885-r",
                    part_of_speech: Adverb,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "in the Christian era; used before dates after the supposed year Christ was born",
                        },
                    ],
                    ili_definition: None,
                    relations: [],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "in AD 200",
                        },
                    ],
                },
            ]
        "#]],
    );

    check(
        &lexicon.synsets[lexicon.synsets.len() - 5..],
        expect![[r#"
            [
                Synset {
                    id: "oewn-92471097-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "an art style in late 16th century Europe characterized by spatial incongruity and excessive elongation of the human figures.",
                        },
                    ],
                    ili_definition: Some(
                        IliDefinition {
                            status: None,
                            note: None,
                            value: "an art style in late 16th century Europe characterized by spatial incongruity and excessive elongation of the human figures.",
                        },
                    ),
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-04936599-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "Mannerism favors compositional tension and instability rather than the balance and clarity of earlier Renaissance painting.",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92471179-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "a surface generated by a moving straight line with the result that through every point on the surface a line can be drawn lying wholly in the surface.",
                        },
                    ],
                    ili_definition: Some(
                        IliDefinition {
                            status: None,
                            note: None,
                            value: "a surface generated by a moving straight line with the result that through every point on the surface a line can be drawn lying wholly in the surface.",
                        },
                    ),
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-04369112-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "In algebraic geometry, ruled surfaces were originally defined as projective surfaces in projective space containing a straight line through any given point.",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92471253-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "a pentagon in which the angles are all equal, and the sides all equal.",
                        },
                    ],
                    ili_definition: Some(
                        IliDefinition {
                            status: None,
                            note: None,
                            value: "a pentagon in which the angles are all equal, and the sides all equal.",
                        },
                    ),
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-13904858-n",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-13889754-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "A regular pentagon has five lines of reflectional symmetry, and rotational symmetry of order 5 (through 72°, 144°, 216° and 288°).",
                        },
                    ],
                },
                Synset {
                    id: "oewn-92767020-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "an electrical device used to create artificial light by use of an electric lamp; all light fixtures have a fixture body and a light socket to hold the lamp and allow for its replacement",
                        },
                    ],
                    ili_definition: Some(
                        IliDefinition {
                            status: None,
                            note: None,
                            value: "an electrical device used to create artificial light by use of an electric lamp; all light fixtures have a fixture body and a light socket to hold the lamp and allow for its replacement",
                        },
                    ),
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-04270870-n",
                            status: None,
                            note: None,
                        },
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-03274312-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [],
                },
                Synset {
                    id: "oewn-92767095-n",
                    part_of_speech: Noun,
                    status: None,
                    note: None,
                    definitions: [
                        Definition {
                            status: None,
                            note: None,
                            value: "a trained person hired to determine the sex of chicken and other hatchlings.",
                        },
                    ],
                    ili_definition: Some(
                        IliDefinition {
                            status: None,
                            note: None,
                            value: "a trained person hired to determine the sex of chicken and other hatchlings.",
                        },
                    ),
                    relations: [
                        SynsetRelation {
                            rel_type: Hypernym,
                            target_id: "oewn-10451389-n",
                            status: None,
                            note: None,
                        },
                    ],
                    examples: [
                        Example {
                            status: None,
                            note: None,
                            value: "Chick sexing is the method of distinguishing the sex of chicken and other hatchlings, usually by a trained person called a chick sexer or chicken sexer.",
                        },
                    ],
                },
            ]
        "#]],
    );
}
