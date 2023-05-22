use crate::Quote;

pub fn seed_data() -> Vec<Quote> {
    vec![
        Quote {
            id: 1,
            author: String::from("Henry van Dijk"),
            content: String::from("Use what talents you have - the woods will be very silent if no birds sang there except those that sang best."),
            category: String::from("Motivational"),
        },
        Quote {
            id: 2,
            content: String::from("People don't think about you as much as you imagine they do."),
            author: String::from("Anonymous"),
            category: String::from("Self-help"),
        },
        Quote {
            id: 3,
            content: String::from("A ship in harbour is safe, but that is not what ships are meant for."),
            author: String::from("John Shedd"),
            category: String::from("Insightful"),
        },
        Quote {
            id: 4,
            author: String::from("C. S. Lewis"),
            content: String::from("The pain I feel now is the happiness I had before. That's the deal."),
            category: String::from("Life"),
        },
        Quote {
            id: 5,
            author: String::from("Anonymous"),
            content: String::from("Let go or be dragged"),
            category: String::from("Life"),
        },
        Quote {
            id: 6,
            author: String::from("Nisargadatta Maharaj"),
            content: String::from("Wisdom tells me I am nothing. Love tells me I am everything. And between the two my life flows."),
            category: String::from("Life"),
        },
        Quote {
            id: 7,
            category: String::from("Insightful"),
            content: String::from("Honesty without kindness is brutality. Kindness without honesty is manipulation."),
            author: String::from("Bryan Reeves"),
        },
        Quote {
            id: 8,
            author: String::from("Dita Von Teese"),
            content: String::from("You might be the sweetest peach on the tree, but some people just don’t like peaches."),
            category: String::from("Life")
        },
        Quote {
            id: 9,
            author: String::from("Mark Twain"),
            content: String::from("I've had a lot of worries in my life. Most of which never happened."),
            category: String::from("Life")
        },
        Quote {
            id: 10,
            author: String::from("Mark Philip D'Souza"),
            content: String::from("I've prayed to all the Gods, and the one that paid heed to my petitions was money."),
            category: String::from("Insightful")
        }, 
        Quote {
            id: 11,
            author: String::from("Della Ida Conceicao"),
            content: String::from("I find myself too busy to be bothered by trivial things that our society considers disruptive."),
            category: String::from("Life")
        },
        Quote {
            id: 12,
            author: String::from("Jordan Peterson"),
            content: String::from("Compare yourself to who you were yesterday, not who someone else is today."),
            category: String::from("Motivational"),
        },
    ]
}

    