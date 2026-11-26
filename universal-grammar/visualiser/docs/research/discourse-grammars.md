# Discourse grammars: prior art for the grammar workbench

**Status: final, 2026-11-17.** The survey resumed after an upstream rate limit. The Silences section lists the searches that found nothing. The systems table marks the facts that this report did not check.

**Topic:** grammars and structural models for discourse, and the critiques of each. The questions that matter most are Q1, Q2, Q3 and Q5.

## Findings that change decisions

- **Q1, Q6.** Black and Wilensky asked whether a story grammar generates all simple stories and only stories ([Black and Wilensky 1979](https://onlinelibrary.wiley.com/doi/abs/10.1207/s15516709cog0303_2)). A tracer over real episodes tests only the first half. Inference: the workbench also needs a check for over-generation, such as generated sample sequences or episodes marked as counterexamples.
- **Q1.** Story grammars written as context-free rewrite rules left the field within about ten years ([Cohn 2013](https://onlinelibrary.wiley.com/doi/10.1111/cogs.12016)). The critics said that the rules added nothing to models of story content ([Black and Wilensky 1979](https://onlinelibrary.wiley.com/doi/abs/10.1207/s15516709cog0303_2), [Wilensky 1983](https://www.cambridge.org/core/journals/behavioral-and-brain-sciences/article/abs/story-grammars-versus-story-points/5DA9B2DFF9F8811942256C97230304B9)). Inference: the workbench must present a grammar as a hypothesis under test, not as a model of the reader.
- **Q3.** In the Discourse Graphbank, 41.22% of nodes have more than one incoming relation ([Wolf and Gibson 2005](https://aclanthology.org/J05-2005.pdf), p. 279). Later studies trace most of this complexity to anaphora, attribution and annotation design ([Lee et al. 2006](https://www.research.ed.ac.uk/en/publications/complexity-of-dependencies-in-discourse-are-dependencies-in-disco/), [Egg and Redeker 2010](https://aclanthology.org/L10-1547/)). URG does not require episodes to form a strict tree ([`docs/milestones/vertical-slice.md`](../../../core/docs/milestones/vertical-slice.md), line 136). Inference: a grammar describes the moves inside one episode, and a separate graph holds the overlaps and relations between episodes.
- **Q3.** Move corpora report optional moves, cycles of moves and free order. This report found no move nested inside an instance of the same move. Inference: the right-regular path of the request covers these structures. Move analysts use "move embedding" for one sentence that realises two moves ([Discussion-section study](https://fhssjournal.org/index.php/ojs/article/download/346/324/610)). The tracer therefore needs a way to give one unit two move labels.
- **Q2.** Hasan's notation for genre structure has the operators of the proposed EBNF, and it adds free order (Halliday and Hasan 1989, through [secondary studies](https://www.researchgate.net/publication/384433394_Analyzing_the_Generic_Structure_Potential_of_Admission_Notifications_of_Universities_of_Pakistan)). EBNF needs two alternatives for two moves in either order, and six for three moves. Inference: keep the EBNF as the fallback. Add a free-order operator only when traced episodes show free order often.
- **Q2, Q4.** Systemic genre studies wrote genre structure as a text formula and also drew flowcharts and networks. The formula is still in use after about 40 years, and Ventola found dynamic networks very difficult ([Ventola 1989](https://www.tandfonline.com/doi/pdf/10.1080/00437956.1989.11435801)). Inference: the grammar text must stay the primary artefact, and the diagram must stay a view that the tool derives from the text.
- **Q1, Q5.** URG keeps alternative readings side by side ([CONTEXT.md](../../../core/CONTEXT.md), "Alternative reading"). Das, Stede and Taboada called the "single ground truth" assumption essentially invalid for RST ([Das et al. 2017](https://aclanthology.org/W17-3602/)). Inference: a trace input can be a set of alternative move sequences. The tracer must report a result for each alternative and must not merge the results.
- **Q2.** URG move labels are provisional ([CONTEXT.md](../../../core/CONTEXT.md), "Move"), and URG rejects a predefined catalogue of rhetorical moves ([`docs/adr/emergent-categories-and-representation.md`](../../../core/docs/adr/emergent-categories-and-representation.md)). Inference: the notation must make a terminal easy to rename, and the tracer must report labels that no rule mentions.
- **Q5, Q6.** A URG pattern must name its supporting instances, its exceptions and its unresolved cases ([CONTEXT.md](../../../core/CONTEXT.md), "Pattern"). RST-Tace compares two analyses one dimension at a time, so that one mismatch does not hide the other matches ([Wan et al. 2019](https://aclanthology.org/W19-2712/)). Inference: the workbench must count the episodes that each rule accepts and rejects. For a rejected episode, it must show the moves that still match.
- **Q3, Q4.** The call-stack inspector of the request draws the same picture as the focus stack of Grosz and Sidner ([Grosz and Sidner 1986](https://dash.harvard.edu/bitstream/handle/1/2579648/Grosz_AttentionIntentions.pdf)). Walker gave discourse examples that the stack model does not predict ([Walker 1996](https://aclanthology.org/J96-2005.pdf)). Inference: the inspector must label the stack as parser state for one grammar, not as a model of reader attention.

## Q1 Purpose

### Story grammars

Story grammars modelled how readers understand and recall simple stories. Rumelhart (1975), Mandler and Johnson (1977), Thorndyke (1977) and Stein and Glenn (1979) wrote rewrite rules over categories such as setting, episode, attempt and outcome. Early recall studies supported the grammars. Readers recalled the higher nodes of the hierarchy better. They also understood text in grammar order better than reorganised text ([technical report](https://www.ideals.illinois.edu/items/18046/bitstreams/64610/data.pdf), [Mandler and Johnson 1977](https://www.researchgate.net/publication/242529824_Remembrance_of_things_parsed_Story_structure_and_recall)).

Black and Wilensky (1979) judged the grammars on formal adequacy, empirical adequacy and value as a model of comprehension ([abstract](https://onlinelibrary.wiley.com/doi/abs/10.1207/s15516709cog0303_2)). They found only one grammar formally adequate. They asked whether each grammar generates all simple stories and only stories. They found that the grammars added nothing to semantic models of story content.

Mandler and Johnson (1980) replied that the critics did not show that discontinuous elements occur in stories ([reply](https://www.sciencedirect.com/science/article/abs/pii/S0364021380800061)). Rumelhart (1980) and Frisch and Perlis (1981) also replied ([Rumelhart 1980](https://www.sciencedirect.com/science/article/abs/pii/S0364021380800073)). Wilensky (1983) then called the analogy between story grammars and sentence grammars "ill conceived" and a "category error" ([Wilensky 1983](https://www.cambridge.org/core/journals/behavioral-and-brain-sciences/article/abs/story-grammars-versus-story-points/5DA9B2DFF9F8811942256C97230304B9)). Garnham (1983) added a further critique ([PubMed](https://pubmed.ncbi.nlm.nih.gov/6686506/)). Cohn (2013) wrote that story grammars "quickly disappeared from the theoretical landscape due to criticism" ([Cohn 2013](https://onlinelibrary.wiley.com/doi/10.1111/cogs.12016)). Garnham (2021) called the debate "unilluminating", and said that the grammars did not generalise beyond one text type ([search snippet of Garnham 2021](https://aclanthology.org/2021.dnd-12.10.pdf)).

Walk-away check: the rules did not survive as a model of comprehension. Inference: the category names stay legible, because each name describes a function in the story.

### Text grammars and macrostructures

Kintsch and van Dijk (1978) separated the local microstructure of a text from its global macrostructure ([ERIC record](https://eric.ed.gov/?id=EJ191792)). A schema of the reader's goals controls the macro-operators, which reduce the text base to its gist. The authors state that macroprocesses can be predicted only when that schema is explicit. Inference: one text can have more than one valid macrostructure. That supports the URG definition of an episode by its purpose. Walk-away check: the split between local and global structure stays legible without the model.

### Rhetorical Structure Theory

Mann and Thompson (1988) built RST as a functional theory of text organisation. Four constraints govern an RST analysis: completeness, connectedness, uniqueness and adjacency ([Potter 2018](https://journals.uic.edu/ojs/index.php/dad/article/download/10690/9465)). Together they make every analysis a tree that covers the whole text ([Zeldes et al. 2024](https://arxiv.org/abs/2403.13560), §2.1). The RST Discourse Treebank then made RST trees the training data for discourse parsers ([Carlson et al. 2001](https://aclanthology.org/W01-1605/)).

Walk-away check: the relation names, such as elaboration, evidence and concession, stay legible without any tool.

### Argumentative Zoning

Teufel and Moens (2002) labelled each sentence of a scientific article with one of seven zones, for summarisation ([Teufel and Moens 2002](https://aclanthology.org/J02-4002.pdf)). The scheme was stable (K = .82, .81, .76) and reproducible (K = .71) ([Teufel 2004](https://cdn.aaai.org/Symposia/Spring/2004/SS-04-07/SS04-07-031.pdf)). The zones had a skewed distribution, with 67% of sentences in the zone OWN. Inference: zoning labels units but states no order of zones. A move grammar adds the order, and so it makes a stronger claim than the labels alone. Walk-away check: the annotated corpus is still available ([AZ corpus](https://www.cl.cam.ac.uk/~sht25/AZ_corpus.html)).

### Explanation genres

Martin and Rose (2008) group explanations into a genre family of the "Sydney School" ([Equinox](https://api.equinoxpub.com/books/1548)). Sequential explanations explain a sequence, factorial explanations explain several causes, and consequential explanations explain several effects ([Rose 2008](https://www.researchgate.net/publication/292655008_Writing_as_linguistic_mastery_The_development_of_genre-based_literacy_pedagogy)). An ERIC study writes the stages with a caret for fixed order, for example "Phenomenon Identification ^ Explanation Sequence" ([ERIC study](https://files.eric.ed.gov/fulltext/EJ1242986.pdf)). The sources disagree on the set of types. School frameworks add causal, cyclical and system explanations ([Jones and Derewianka](https://en.unipress.dk/media/14675/sprogforum_63_jones_derewianka.pdf)). A teaching handout warns that the genre of an explanation is sometimes not clear-cut ([University of Wollongong](https://documents.uow.edu.au/content/groups/public/@web/@educ/documents/doc/uow164239.pdf)).

### Explanation in science teaching

Ogborn, Kress, Martins and McGillicuddy (1996) studied video recordings of secondary school teachers who explained science ([Dini et al. 2020](https://onlinelibrary.wiley.com/doi/10.1002/sce.21559)). They describe four parts of an explanation. The parts are the creation of differences, the construction of entities, the transformation of knowledge, and demonstrations that put meaning into matter ([SciELO article](http://www.scielo.br/j/epec/a/mhHjW5VtZXgStN6r53XKZHs/?lang=pt)). Inference: the sources state no fixed order for these parts. They are candidate move labels for URG, not a grammar.

### Move analysis

Swales built move analysis to describe and teach the structure of research article introductions. His CARS model ("Create a Research Space") claims to represent only "high probability expectations" (Swales 1990, p. 58, as quoted in [a 2017 study of thesis introductions](https://www.sciencedirect.com/science/article/abs/pii/S147515851730125X)). Biber, Connor and Upton (2007) turned move analysis into a corpus method of seven steps ([summary of the method](https://scholarworks.indianapolis.iu.edu/server/api/core/bitstreams/c45917c7-280e-4b33-9440-ae1a88aa5fee/content)). The first step gives each move type an operational definition. Kanoksilapatham (2005) applied the method to 60 biochemistry research articles, with a coding reliability analysis of the move boundaries ([Kanoksilapatham 2005](https://www.sciencedirect.com/science/article/abs/pii/S0889490604000705)).

Anthony (1999) tested the CARS model on 12 software engineering articles that won "best paper" awards ([Anthony 1999](https://eprints.lancs.ac.uk/id/eprint/71622/)). The model described the main framework, but "a number of important features are not accounted for". Citing studies name long reviews of background literature, definitions and examples ([Semantic Scholar record](https://www.semanticscholar.org/paper/Writing-research-article-introductions-in-software-Anthony/b12aaa8061d49f089462360cd77b0af57d85e38c)).

Mover labels each sentence of an abstract with a move, for non-native writers ([Anthony and Lashkia 2003](https://www.researchgate.net/publication/3230281_Mover_A_Machine_Learning_Tool_to_Assist_in_the_Reading_and_Writing_of_Technical_Papers)). It uses a Naive Bayes classifier, trained on 100 abstracts from information technology. Wu and colleagues tagged five moves in abstracts with a hidden Markov model, for a concordancer called CARE ([Wu et al. 2006](https://aclanthology.org/P06-4011/)).

Walk-away check: writing guides still teach the CARS moves ([USC research guide](https://libguides.usc.edu/writingguide/CARS)). The model stays legible without software, because it is a short list of named moves and steps.

**Fallback to beat:** the purpose in the request, a tracer that accepts or rejects real episodes. Evidence to depart: the over-generation test of Black and Wilensky already justifies one addition, a list of sample sequences that the grammar generates.

## Q2 Notation

### Generic structure formulas

Hasan wrote the structure of a genre as one formula, the generic structure potential. Her formula for a shop transaction is `[(G).(SI)^][(SE.){SR^SC^}^S^]P^PC(^F)` (Halliday and Hasan 1989, pp. 63–65). Secondary studies give these meanings for the symbols:

- Round brackets mark an optional element.
- A caret marks fixed order.
- A dot marks an element that can take more than one position.
- Square brackets mark the range inside which an element can move.
- Braces with a curved arrow mark iteration, with an equal degree of iteration for the enclosed elements.

The sources disagree on what the braces cover. The curved arrows do not survive in plain text, and later studies add symbols such as `/` and `<>`. This report saw the formula only in search snippets of secondary studies. Studies from 2021 to 2024 still apply the notation, for example to admission notifications, sermons and dating sites ([example](https://www.researchgate.net/publication/384433394_Analyzing_the_Generic_Structure_Potential_of_Admission_Notifications_of_Universities_of_Pakistan)).

Walk-away check: the formula is plain text, and it stayed legible for about 40 years without any tool. Inference: the formula has the operators of the proposed EBNF, and it adds free order.

### Flowcharts and networks

Martin (1985) separated two views of a genre ([O'Donnell, "Context in Dynamic Modelling"](http://www.wagsoft.com/Papers/ODonContext.pdf)). A synoptic model shows the overall shape of an interaction, and a dynamic model shows the interaction as it unfolds. O'Donnell reports that one synoptic diagram became the "lingua franca" for work on exchanges. Ventola (1987) drew a flowchart for service encounters, with diamonds for choices and squares for the realisation of an element.

Ventola (1989) compared a flowchart, a transition network and a dynamic network ([Ventola 1989](https://www.tandfonline.com/doi/pdf/10.1080/00437956.1989.11435801)). Her flowchart covered only the Australian service encounters that she collected. She found generic structures very difficult to represent with dynamic networks. She saw "a strong didactic potential" in the flowchart. This report saw only fragments of a broken scan.

Inference: the request proposes both views, rules as text and the state machine as a diagram. This tradition tried both views, and the text formula outlived the diagrams.

**Fallback to beat:** the EBNF of the request. Evidence to depart: traced episodes that need many alternatives only to express free order. In that case, add a free-order operator such as the square brackets of Hasan.

## Q3 Semantics

### Trees, graphs and crossing relations

Wolf and Gibson annotated 135 texts from the Wall Street Journal and the AP Newswire ([Wolf and Gibson 2005](https://aclanthology.org/J05-2005.pdf)). They found "many different kinds of crossed dependencies, as well as many nodes with multiple parents". They concluded that trees cannot describe discourse coherence.

In their graphs, 41.22% of the nodes have an in-degree greater than one (p. 279, seen in a search snippet). Egg and Redeker (2010) cite a second figure from p. 273. On average, the removal of 12.5% of the relations eliminates all crossed dependencies.

Two sources disagree with Wolf and Gibson. Lee and colleagues found nested, crossed and other non-tree configurations in the Penn Discourse Treebank ([Lee et al. 2006](https://www.research.ed.ac.uk/en/publications/complexity-of-dependencies-in-discourse-are-dependencies-in-disco/)). They concluded that "the types of discourse dependencies are highly restricted", because anaphora and attribution explain the complex cases. Egg and Redeker argued that "much of the additional complexity in the DGB is not inherent in the data, but due to specific design choices" ([Egg and Redeker 2010](https://aclanthology.org/L10-1547/)). A tree can still hold crossings from lexical or referential overlap, many-parent nodes under the Nuclearity Principle of Marcu (1996), and lists.

The Penn Discourse Treebank does not take a side. Its authors call the annotation "theory-neutral" with respect to higher-level discourse structure ([Prasad et al. 2008](http://www.lrec-conf.org/proceedings/lrec2008/pdf/754_paper.pdf)). Each relation has a connective and two arguments, and the corpus builds no tree over the relations. A later paper presents the corpus as data for an emergent theory ([Prasad, Webber and Joshi 2014](https://aclanthology.org/J14-4007.pdf)). This report saw both statements in search snippets only.

Enhanced RST (eRST) keeps the ordinary RST tree ([Zeldes et al. 2024](https://arxiv.org/abs/2403.13560), §3.2). It adds secondary edges that can cross, form cycles and connect nodes that the tree already connects. Each secondary edge needs a signal in the text, such as a connective.

Secondary edges are 3.37% of the relations in GUM and the RST-DT test set (§4.2). Agreement on their relation rose from an F score of .223 to .412 after a guideline revision, against .571 for primary relations (§4.1.2). The authors concede that the graphs "are quite complex" (§3.4).

Inference: the dispute concerns relations across a whole text. A URG episode is smaller, and URG keeps relationships as a separate record. eRST shows the same split in practice: a tree for the main structure, and a separate layer for the relations that break it.

### Discontinuous elements

Black and Wilensky said that context-free grammars cannot handle discontinuous elements. Mandler and Johnson answered that the critics did not show that such elements occur in stories ([reply](https://www.sciencedirect.com/science/article/abs/pii/S0364021380800061)). Inference: interleaved moves are the case where a sequence grammar fails. The workbench must show such a failure as a rejection, and must not hide it.

### Repetition or embedding in move sequences

Move studies report repeated cycles of moves. The CARS model allows the "recycling" of Move 2 (Swales and Feak 2012, as cited in [a study of thesis introductions](https://www.sciencedirect.com/science/article/abs/pii/S147515851730125X)). Swales (2004) revised the model to allow cyclical patterns ([ERIC study](https://files.eric.ed.gov/fulltext/EJ1070213.pdf)). Spanish computing theses show frequent cycles of Move 1 with Move 2, and of Move 2 with Move 3 ([thesis study](https://www.researchgate.net/publication/258049612_Spanish_PhD_theses_in_computing_A_study_of_the_schematic_structure_of_the_introduction_chapters)). One annotation manual tells annotators that "The moves are not necessarily in order" ([USI manual](https://ssl.lu.usi.ch/entityws/Allegati/3034503_638736706315121060.pdf)).

Wu and colleagues modelled move order with the transitions of a hidden Markov model, which is a finite-state model ([Wu et al. 2006](https://aclanthology.org/P06-4011/)). Their best setting gave the transition probabilities a weight of 0.7 and reached a precision of 80.54%, according to a [secondary summary](https://antonetteshibani.com/research-article-notes/notes-computational-analysis-of-move-structures-in-academic-abstracts/).

This report found no corpus study with a move inside another instance of the same move. Move analysts use "move embedding" for one sentence that realises two moves ([Discussion-section study](https://fhssjournal.org/index.php/ojs/article/download/346/324/610)). A study of abstracts reports it most often between Purpose and Method ([abstracts study](https://www.researchgate.net/publication/343181384_International_Journal_of_Linguistics_and_Translation_Studies)). Story grammars did allow an episode inside an episode, but this report takes that point from memory and did not check it.

Inference: repetition, optional elements and free order cover the reported move structures, and all three are regular. Self-embedding in explanation episodes would be a new finding, and the workbench must be able to show it. An embedded move breaks the model of one token for each unit.

### Stacks and attention

Grosz and Sidner (1986) modelled discourse with an intentional structure and an attentional state ([Grosz and Sidner 1986](https://dash.harvard.edu/bitstream/handle/1/2579648/Grosz_AttentionIntentions.pdf)). The intentional structure is a tree of segment purposes. The attentional state is a stack of focus spaces. A segment's focus space goes onto the stack when its purpose contributes to the purpose at the top. Webber later showed that attachment on the right frontier of a discourse tree matches these stack operations ([Webber](https://arxiv.org/pdf/cmp-lg/9708003)).

Walker (1996) gave examples that the stack model does not predict ([Walker 1996](https://aclanthology.org/J96-2005.pdf)). She argued that the stack model does not relate to known limits on working memory, and proposed a cache of limited capacity. The cache model explains why speakers repeat information that both parties already know. Inference: a reader can mistake a call-stack inspector for a model of attention.

**Fallback to beat:** the Glushkov automaton and the recursive transition network of the request. Evidence to depart: interleaved moves inside one episode, which no sequence grammar accepts. Evidence to cut the recursive path: no self-embedding in the traced episodes.

## Q4 Layout and interaction

RSTTool builds an RST diagram as the analyst drags from segment to segment, and exports the diagram as EPS ([O'Donnell 2000](https://aclanthology.org/W00-1434/)). The ANNIS view of RST trees only shows corpora that someone annotated earlier (Krause and Zeldes 2016, as cited by the [TIARA 2.0 paper](https://www.researchgate.net/publication/356509540_TIARA_20_An_Interactive_Tool_for_Annotating_Discourse_Structure_and_Text_Improvement)). TIARA 2.0 lets the annotator collapse subtrees. Its dual view shows the logical sequence of the text beside the overall structure. Its authors claim an advantage for longer texts, where clutter lowers the quality of annotation. TreeAnnotator also offers two view modes ([TreeAnnotator](https://aclanthology.org/L18-1308.pdf)).

This report found no study that measured how well readers understand RST diagrams. For genre diagrams, one synoptic diagram became a shared language, and Ventola saw a didactic value in her flowchart (see Q2). Inference: the drill-down of the request opens one non-terminal at a time, as TIARA collapses subtrees. A second view can show the traced episode as text, with each move marked.

**Fallback to beat:** the layered layout with drill-down of the request. Evidence to depart: grammars whose top-level diagram does not fit on one screen.

## Q5 Scope

### Agreement between annotators

The RST Discourse Treebank holds 385 Wall Street Journal articles ([Carlson et al. 2001](https://aclanthology.org/W01-1605/)). Two or more professional analysts tagged 53 of the articles (13.8%). The team measured kappa for segments, spans, nuclearity and relations. The paper treats a kappa above 0.8 as very high agreement, and 0.6 to 0.8 as good. This report did not see the figures in Table 1.

Wolf and Gibson reported a kappa above 0.8 for the Discourse Graphbank ([Wolf and Gibson 2005](https://aclanthology.org/J05-2005.pdf)).

### Alternative analyses

Das, Stede and Taboada (2017) compared parallel RST analyses of German and English texts by expert annotators ([Das, Stede and Taboada 2017](https://www.sfu.ca/~mtaboada/docs/research/Das_et_al_2017_RST_workshop.pdf), [ACL Anthology](https://aclanthology.org/W17-3602/)). They separated legitimate disagreements, which come from different possible readings of the text, from errors. They proposed an underspecified "complex gold" annotation that holds the alternative expert trees (§5.2). rstWeb also stores more than one annotated version of each document ([Zeldes 2016](https://aclanthology.org/N16-3001/)). Inference: RST practice moved in the same direction as URG. The tracer must accept alternative sequences for one episode, and must show which alternatives the grammar accepts.

**Fallback to beat:** one move sequence for each trace. The URG alternative readings and the "complex gold" proposal both justify a set of alternative sequences instead.

## Q6 Test seams

RST-Tace compares two RST analyses of one text ([Wan et al. 2019](https://aclanthology.org/W19-2712/)). It implements the comparison method of Iruskieta, which compares constituent, attachment point, nuclearity and relation, each independent of decisions lower in the tree. The tool reports an F-measure and the agreement between annotators, and it writes the comparison table automatically. Inference: the tracer can compare the derivation of an episode with the annotated moves in the same way, one dimension at a time.

**Fallback to beat:** one accept or reject result for each trace. Evidence to depart: URG patterns need counts of supporting instances and exceptions, so the workbench must also sum the results over all episodes.

## Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance | Walk-away result |
|---|---|---|---|---|---|---|
| RST Discourse Treebank | <https://aclanthology.org/W01-1605/> | Corpus of RST trees over 385 news articles | LDC licence (LDC2002T07) | 2002 | Static corpus | The relation names stay legible. This report did not check the file format |
| Discourse Graphbank | <https://catalog.ldc.upenn.edu/LDC2005T08> | Coherence graphs over 135 news texts of 70,000 words | LDC licence (LDC2005T08) | 15 March 2005 | Static corpus | Its claim that trees cannot hold coherence stays legible, and stays disputed |
| Penn Discourse Treebank 2.0 | <https://aclanthology.org/L08-1093/> | Relations with connectives over one million words of news text | LDC licence (LDC2008T05) | 2008. This report did not check later versions | Not checked | Each relation names a connective and two arguments in the text |
| rstWeb | <https://github.com/amir-zeldes/rstWeb> | Browser editor for RST trees, signals and, in eRST, secondary edges | MIT | No tagged release. The repository shows 232 commits, 48 stars and 14 open issues | Extended in 2019 and 2024 | The `.rs3` XML files stay readable without the tool |
| RSTTool | <https://aclanthology.org/W00-1434/> | Desktop tool for the manual RST analysis of a text | Not checked | Version 2.4 (2000) | Not checked | Its `.rs3` XML format outlived it, because rstWeb imports the files |
| ANNIS with eRST | <https://arxiv.org/abs/2403.13560> | Search of eRST corpora, with a view of each analysis | Apache 2.0 | Not checked | Active in 2024 | Not checked |
| RST-Tace | <https://aclanthology.org/W19-2712/> | Comparison of two RST analyses of one text | Not checked | 2019 | Not checked | The comparison method of Iruskieta stays legible without the tool |
| TIARA 2.0 | <https://www.researchgate.net/publication/356509540_TIARA_20_An_Interactive_Tool_for_Annotating_Discourse_Structure_and_Text_Improvement> | Annotation of discourse structure, with collapsible trees | Not checked | Not checked | Not checked | Not checked |
| Mover (AntMover) | <http://www.laurenceanthony.net/software/antmover/> | Move labels for the sentences of an abstract | Freeware | Not checked | Not checked | Its output is plain text files, one for each move type |
| CARE | <https://aclanthology.org/P06-4011/> | Concordancer over abstracts with move tags | Not checked | Prototype (2006) | Not checked | The five move labels stay legible |

## Silences

- The first four web searches failed on an upstream rate limit, not on an absence of results. They are not silences.
- Query "Wolf Gibson 2005 "Representing discourse coherence: A corpus-based study" crossed dependencies nodes multiple parents percent" returned no figures. A second query found the 41.22% figure.
- Query "Lee Prasad Joshi Dinesh Webber 2006 "Complexity of dependencies in discourse" Penn Discourse Treebank shared arguments crossing" returned no counts.
- Query "Taboada Mann 2006 "Rhetorical Structure Theory: looking back and moving ahead" Discourse Studies criticisms tree constraint" returned no text on the tree constraint.
- Query "Mann Thompson Rhetorical Structure Theory constraints completeness connectedness uniqueness adjacency "multiple analyses" sfu.ca/rst" found the four constraints. It did not find the section of Mann and Thompson on multiple analyses. A snippet says that later papers (Mann et al. 1992, Mann 2002) weakened the uniqueness constraint. This report did not trace the snippet to its page.
- Query "RST tree diagrams hard to read long texts visualization rhetorical structure theory annotation tool display" found no study of how readers understand RST diagrams. The closest source is the TIARA 2.0 paper, which claims less clutter for long texts.
- Query ""move embedding" OR "embedded moves" genre move analysis research articles corpus Swales" found no move nested inside the same move. It found "move embedding" only in the sense of two moves in one sentence.

## Sources

- Black, J. B. and Wilensky, R. (1979). An evaluation of story grammars. *Cognitive Science* 3(3), 213–229. <https://onlinelibrary.wiley.com/doi/abs/10.1207/s15516709cog0303_2>
- Carlson, L., Marcu, D. and Okurowski, M. E. (2001). Building a discourse-tagged corpus in the framework of Rhetorical Structure Theory. SIGdial 2001. <https://aclanthology.org/W01-1605/>
- Cohn, N. (2013). Visual narrative structure. *Cognitive Science* 37(3). <https://onlinelibrary.wiley.com/doi/10.1111/cogs.12016>
- Egg, M. and Redeker, G. (2010). How complex is discourse structure? LREC 2010, 1619–1623. <https://aclanthology.org/L10-1547/>
- Garnham, A. (1983). What's wrong with story grammars. *Cognition* 15, 145–154. <https://pubmed.ncbi.nlm.nih.gov/6686506/>
- Garnham, A. (2021). *Dialogue and Discourse* 12(1). <https://aclanthology.org/2021.dnd-12.10.pdf>
- Lee, A., Prasad, R., Joshi, A., Dinesh, N. and Webber, B. (2006). Complexity of dependencies in discourse. TLT 5, 79–90. <https://www.research.ed.ac.uk/en/publications/complexity-of-dependencies-in-discourse-are-dependencies-in-disco/>
- Mandler, J. M. and Johnson, N. S. (1977). Remembrance of things parsed. *Cognitive Psychology* 9, 111–151. <https://www.researchgate.net/publication/242529824_Remembrance_of_things_parsed_Story_structure_and_recall>
- Mandler, J. M. and Johnson, N. S. (1980). On throwing out the baby with the bathwater. *Cognitive Science* 4, 305–312. <https://www.sciencedirect.com/science/article/abs/pii/S0364021380800061>
- Rumelhart, D. E. (1980). On evaluating story grammars. *Cognitive Science* 4(3), 313–316. <https://www.sciencedirect.com/science/article/abs/pii/S0364021380800073>
- Taboada, M. and Mann, W. C. (2006). Rhetorical Structure Theory: looking back and moving ahead. *Discourse Studies* 8(3), 423–459. <https://journals.sagepub.com/doi/10.1177/1461445606061881>
- Wilensky, R. (1983). Story grammars versus story points. *Behavioral and Brain Sciences* 6(4). <https://www.cambridge.org/core/journals/behavioral-and-brain-sciences/article/abs/story-grammars-versus-story-points/5DA9B2DFF9F8811942256C97230304B9> <!-- tc: depart spelling because the journal title is a proper name -->
- Wolf, F. and Gibson, E. (2005). Representing discourse coherence: a corpus-based study. *Computational Linguistics* 31(2), 249–287. <https://aclanthology.org/J05-2005.pdf>
- Anthony, L. (1999). Writing research article introductions in software engineering: how accurate is a standard model? *IEEE Transactions on Professional Communication* 42(1), 38–46. <https://eprints.lancs.ac.uk/id/eprint/71622/>
- Biber, D., Connor, U. and Upton, T. A. (2007). *Discourse on the Move*. John Benjamins. <https://benjamins.com/catalog/scl.28.04kan> (the page of one chapter of the book)
- Grosz, B. J. and Sidner, C. L. (1986). Attention, intentions, and the structure of discourse. *Computational Linguistics* 12(3), 175–204. <https://dash.harvard.edu/bitstream/handle/1/2579648/Grosz_AttentionIntentions.pdf>
- Halliday, M. A. K. and Hasan, R. (1989). *Language, Context, and Text*. Oxford University Press. No URL. Seen only through secondary studies.
- Secondary study of the generic structure potential of university admission notifications in Pakistan. <https://www.researchgate.net/publication/384433394_Analyzing_the_Generic_Structure_Potential_of_Admission_Notifications_of_Universities_of_Pakistan>
- Kanoksilapatham, B. (2005). Rhetorical structure of biochemistry research articles. *English for Specific Purposes* 24(3), 269–292. <https://www.sciencedirect.com/science/article/abs/pii/S0889490604000705>
- O'Donnell, M. Context in dynamic modelling. <http://www.wagsoft.com/Papers/ODonContext.pdf>
- Ventola, E. (1989). Problems of modelling and applied issues within the framework of genre. *Word*. <https://www.tandfonline.com/doi/pdf/10.1080/00437956.1989.11435801>
- Walker, M. A. (1996). Limited attention and discourse structure. *Computational Linguistics* 22(2), 255–264. <https://aclanthology.org/J96-2005.pdf>
- Webber, B. Structure and ostension in the interpretation of discourse deixis. <https://arxiv.org/pdf/cmp-lg/9708003>
- Study of the introductions of applied linguistics PhD theses (2017). <https://www.sciencedirect.com/science/article/abs/pii/S147515851730125X>
- Study of Spanish PhD theses in computing. <https://www.researchgate.net/publication/258049612_Spanish_PhD_theses_in_computing_A_study_of_the_schematic_structure_of_the_introduction_chapters>
- Annotation manual for the CARS model, USI. <https://ssl.lu.usi.ch/entityws/Allegati/3034503_638736706315121060.pdf>
- Das, D., Stede, M. and Taboada, M. (2017). The good, the bad, and the disagreement: complex ground truth in rhetorical structure analysis. 6th Workshop on RST and Related Formalisms, 11–19. <https://aclanthology.org/W17-3602/> Full text: <https://www.sfu.ca/~mtaboada/docs/research/Das_et_al_2017_RST_workshop.pdf>
- O'Donnell, M. (2000). RSTTool 2.4: a markup tool for Rhetorical Structure Theory. INLG 2000, 253–256. <https://aclanthology.org/W00-1434/>
- Zeldes, A. (2016). rstWeb: a browser-based annotation interface for Rhetorical Structure Theory and discourse relations. NAACL 2016 Demonstrations, 1–5. <https://aclanthology.org/N16-3001/> Repository: <https://github.com/amir-zeldes/rstWeb>
- Zeldes, A., Aoyama, T., Liu, Y. J., Peng, S., Das, D. and Gessler, L. (2024). eRST: a signaled graph theory of discourse relations and organization. arXiv:2403.13560v2. <https://arxiv.org/abs/2403.13560> <!-- tc: depart spelling because the paper title is a proper name -->
- Teufel, S. and Moens, M. (2002). Summarizing scientific articles: experiments with relevance and rhetorical status. *Computational Linguistics* 28(4), 409–446. <https://aclanthology.org/J02-4002.pdf> <!-- tc: depart spelling because the paper title is a proper name -->
- Teufel, S. (2004). Argumentative zoning for improved citation indexing. AAAI Spring Symposium. <https://cdn.aaai.org/Symposia/Spring/2004/SS-04-07/SS04-07-031.pdf>
- Martin, J. R. and Rose, D. (2008). *Genre Relations: Mapping Culture*. Equinox. <https://api.equinoxpub.com/books/1548>
- Rose, D. (2008). Writing as linguistic mastery. <https://www.researchgate.net/publication/292655008_Writing_as_linguistic_mastery_The_development_of_genre-based_literacy_pedagogy>
- Ogborn, J., Kress, G., Martins, I. and McGillicuddy, K. (1996). *Explaining Science in the Classroom*. Open University Press. Seen through [Dini et al. 2020](https://onlinelibrary.wiley.com/doi/10.1002/sce.21559) and a [SciELO article](http://www.scielo.br/j/epec/a/mhHjW5VtZXgStN6r53XKZHs/?lang=pt).
- Wan, S., Kutschbach, T., Lüdeling, A. and Stede, M. (2019). RST-Tace: a tool for automatic comparison and evaluation of RST trees. DISRPT 2019, 88–96. <https://aclanthology.org/W19-2712/>
- Potter, A. (2018). *Dialogue & Discourse* 9(2), 80–110. doi:10.5087/dad.2018.203. <https://journals.uic.edu/ojs/index.php/dad/article/download/10690/9465>
- Kintsch, W. and van Dijk, T. A. (1978). Toward a model of text comprehension and production. *Psychological Review* 85(5), 363–394. <https://eric.ed.gov/?id=EJ191792>
- Anthony, L. and Lashkia, G. V. (2003). Mover: a machine learning tool to assist in the reading and writing of technical papers. *IEEE Transactions on Professional Communication* 46(3), 185–193. <https://www.researchgate.net/publication/3230281_Mover_A_Machine_Learning_Tool_to_Assist_in_the_Reading_and_Writing_of_Technical_Papers> Software page: <http://www.laurenceanthony.net/software/antmover/>
- Wu, J.-C., Chang, Y.-C., Liou, H.-C. and Chang, J. S. (2006). Computational analysis of move structures in academic abstracts. COLING/ACL 2006 Interactive Presentation Sessions, 41–44. <https://aclanthology.org/P06-4011/>
- Secondary summary of Wu et al. (2006). <https://antonetteshibani.com/research-article-notes/notes-computational-analysis-of-move-structures-in-academic-abstracts/>
- Prasad, R. et al. (2008). The Penn Discourse TreeBank 2.0. LREC 2008. <http://www.lrec-conf.org/proceedings/lrec2008/pdf/754_paper.pdf> ACL Anthology: <https://aclanthology.org/L08-1093/>
- Prasad, R., Webber, B. and Joshi, A. (2014). Reflections on the Penn Discourse TreeBank, comparable corpora, and complementary annotation. *Computational Linguistics* 40(4), 921–950. <https://aclanthology.org/J14-4007.pdf>
- Wolf, F., Gibson, E., Fisher, A. and Knight, M. (2005). Discourse Graphbank, LDC2005T08. <https://catalog.ldc.upenn.edu/LDC2005T08>
- TIARA 2.0: an interactive tool for annotating discourse structure and text improvement. <https://www.researchgate.net/publication/356509540_TIARA_20_An_Interactive_Tool_for_Annotating_Discourse_Structure_and_Text_Improvement>
- TreeAnnotator: versatile visual annotation of hierarchical text relations. LREC 2018. <https://aclanthology.org/L18-1308.pdf>
- Move analysis of senior high school research abstracts in a Philippine university. <https://www.researchgate.net/publication/343181384_International_Journal_of_Linguistics_and_Translation_Studies>
- Comparison of Discussion sections in journal articles and theses. <https://fhssjournal.org/index.php/ojs/article/download/346/324/610>
- Zeldes et al., eRST, also in *Computational Linguistics* (2025). <https://aclanthology.org/2025.cl-1.3.pdf>
- Illinois technical report on story structure and recall. <https://www.ideals.illinois.edu/items/18046/bitstreams/64610/data.pdf>
- Semantic Scholar record of Anthony (1999). <https://www.semanticscholar.org/paper/Writing-research-article-introductions-in-software-Anthony/b12aaa8061d49f089462360cd77b0af57d85e38c>
- Jones, P. and Derewianka, B. *Sprogforum* 63. <https://en.unipress.dk/media/14675/sprogforum_63_jones_derewianka.pdf>
- University of Wollongong teaching handout. <https://documents.uow.edu.au/content/groups/public/@web/@educ/documents/doc/uow164239.pdf>
- ERIC study of the stages of explanations. <https://files.eric.ed.gov/fulltext/EJ1242986.pdf>
- ERIC study of cyclical move patterns. <https://files.eric.ed.gov/fulltext/EJ1070213.pdf>
- USC research guide on the CARS model. <https://libguides.usc.edu/writingguide/CARS>
- Summary of the corpus method of Biber, Connor and Upton (2007). <https://scholarworks.indianapolis.iu.edu/server/api/core/bitstreams/c45917c7-280e-4b33-9440-ae1a88aa5fee/content>
- Teufel, S. Argumentative Zoning corpus. <https://www.cl.cam.ac.uk/~sht25/AZ_corpus.html>
- UniversalGrammar: [`CONTEXT.md`](../../../core/CONTEXT.md)
- UniversalGrammar: [`docs/adr/emergent-categories-and-representation.md`](../../../core/docs/adr/emergent-categories-and-representation.md)
- UniversalGrammar: [`docs/milestones/vertical-slice.md`](../../../core/docs/milestones/vertical-slice.md)

## Decision records

This decision record cites this survey:

- `docs/adr/trees-follow-rules-and-passes.md`: each grammar gives one hierarchy, and relations between steps stay in URG records.
