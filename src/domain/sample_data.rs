use crate::domain::models::*;

pub fn sample_resume() -> Resume {
    Resume {
        personal_info: PersonalInfo {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "123-456-7890".to_string(),
            website: "https://johndoe.dev".to_string(),
            linkedin: "johndoe".to_string(),
            github: "johndoe".to_string(),
            location: "New York, NY".to_string(),
            summary: "Experienced software engineer with a passion for building impactful products. Skilled in designing, developing, and deploying scalable applications using modern technologies. Proven track record of leading teams, mentoring junior developers, and delivering high-quality software on time. Adept at collaborating with cross-functional teams to solve complex problems and drive innovation. Always eager to learn new tools and contribute to open source projects. Seeking opportunities to make a meaningful impact in a dynamic environment."
                .to_string(),
        },
        education: vec![
            Education {
                institution: "State University".to_string(),
                degree: "B.Sc.".to_string(),
                field_of_study: "Computer Science".to_string(),
                start_date: "2015-09".to_string(),
                end_date: "2019-06".to_string(),
                location: "New York, NY".to_string(),
                description: "Graduated with honors.".to_string(),
                gpa: "3.8".to_string(),
            }
        ],
        experience: vec![
            Experience {
                company: "Tech Corp".to_string(),
                position: "Software Engineer".to_string(),
                start_date: "2019-07".to_string(),
                end_date: "2022-08".to_string(),
                location: "Remote".to_string(),
                description: "Worked on backend systems.".to_string(),
                achievements: vec!["Improved API performance by 30%".to_string()],
                is_current: false,
            }
        ],
        skills: Skills {
            categories: Default::default(),
            skill_list: vec![
                Skill { name: "Rust".to_string(), level: 4 },
                Skill { name: "React".to_string(), level: 5 },
            ],
        },
        projects: vec![
            Project {
                name: "Open Source CLI".to_string(),
                role: "Lead Developer".to_string(),
                start_date: None,
                end_date: None,
                description: "A CLI tool for productivity.".to_string(),
                technologies: vec!["Rust".to_string(), "CLI".to_string()],
                url: "https://github.com/johndoe/cli".to_string(),
            }
        ],
        theme: ResumeTheme::Professional,
    }
} 