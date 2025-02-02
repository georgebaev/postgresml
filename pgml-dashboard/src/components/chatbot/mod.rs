use pgml_components::component;
use sailfish::TemplateOnce;

type ExampleQuestions = [(&'static str, [(&'static str, &'static str); 4]); 4];
const EXAMPLE_QUESTIONS: ExampleQuestions = [
    (
        "PostgresML",
        [
            ("How do I", "use pgml.transform()?"),
            ("Show me", "a query to train a model"),
            ("What is HNSW", "indexing"),
            ("Teach me", "how to use pgml.embed()"),
        ],
    ),
    (
        "PyTorch",
        [
            ("What are", "tensors?"),
            ("How do I", "train a model?"),
            ("Show me", "some features of PyTorch"),
            ("Explain", "how to use an optimizer?"),
        ],
    ),
    (
        "Rust",
        [
            ("What is", "a lifetime?"),
            ("How do I", "use a for loop?"),
            ("Show me", "an example of using map"),
            ("Explain", "the borrow checker"),
        ],
    ),
    (
        "PostgreSQL",
        [
            ("How do I", "join two tables?"),
            ("What is", "a GIN index?"),
            ("When should I", "use an outer join?"),
            ("Explain", "what relational data is"),
        ],
    ),
];

const KNOWLEDGE_BASES: [&'static str; 0] = [
    // "Knowledge Base 1",
    // "Knowledge Base 2",
    // "Knowledge Base 3",
    // "Knowledge Base 4",
];

const KNOWLEDGE_BASES_WITH_LOGO: [KnowledgeBaseWithLogo; 4] = [
    KnowledgeBaseWithLogo::new("PostgresML", "/dashboard/static/images/owl_gradient.svg"),
    KnowledgeBaseWithLogo::new("PyTorch", "/dashboard/static/images/logos/pytorch.svg"),
    KnowledgeBaseWithLogo::new("Rust", "/dashboard/static/images/logos/rust.svg"),
    KnowledgeBaseWithLogo::new(
        "PostgreSQL",
        "/dashboard/static/images/logos/postgresql.svg",
    ),
];

struct KnowledgeBaseWithLogo {
    name: &'static str,
    logo: &'static str,
}

impl KnowledgeBaseWithLogo {
    const fn new(name: &'static str, logo: &'static str) -> Self {
        Self { name, logo }
    }
}

const CHATBOT_BRAINS: [ChatbotBrain; 0] = [
    // ChatbotBrain::new(
    //     "PostgresML",
    //     "Falcon 180b",
    //     "/dashboard/static/images/owl_gradient.svg",
    // ),
    // ChatbotBrain::new(
    //     "OpenAI",
    //     "ChatGPT",
    //     "/dashboard/static/images/logos/openai.webp",
    // ),
    // ChatbotBrain::new(
    //     "Anthropic",
    //     "Claude",
    //     "/dashboard/static/images/logos/anthropic.webp",
    // ),
    // ChatbotBrain::new(
    //     "Meta",
    //     "Llama2 70b",
    //     "/dashboard/static/images/logos/meta.webp",
    // ),
];

struct ChatbotBrain {
    provider: &'static str,
    model: &'static str,
    logo: &'static str,
}

// impl ChatbotBrain {
//     const fn new(provider: &'static str, model: &'static str, logo: &'static str) -> Self {
//         Self {
//             provider,
//             model,
//             logo,
//         }
//     }
// }

#[derive(TemplateOnce)]
#[template(path = "chatbot/template.html")]
pub struct Chatbot {
    brains: &'static [ChatbotBrain; 0],
    example_questions: &'static ExampleQuestions,
    knowledge_bases: &'static [&'static str; 0],
    knowledge_bases_with_logo: &'static [KnowledgeBaseWithLogo; 4],
}

impl Chatbot {
    pub fn new() -> Chatbot {
        Chatbot {
            brains: &CHATBOT_BRAINS,
            example_questions: &EXAMPLE_QUESTIONS,
            knowledge_bases: &KNOWLEDGE_BASES,
            knowledge_bases_with_logo: &KNOWLEDGE_BASES_WITH_LOGO,
        }
    }
}

component!(Chatbot);
