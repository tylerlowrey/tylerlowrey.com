use maud::{html, PreEscaped, DOCTYPE};

pub fn index_page() -> String {
    html! {
        (DOCTYPE)
        html {
            head {
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta charset="UTF-8";
                link rel="preload" href="/fonts/EnvyCodeRNFM.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href="/styles/fonts.css";
                link rel="stylesheet" href="/styles/styles.css";
                (PreEscaped("<script src=\"js/wallpaper.js\"></script>"));
            }

            body {
                canvas #background {}
                canvas #wallpaper {}
                #content {
                    #window {
                        pre #name {
                            r#"
                                                                     
         ███████████            ████                                 
        ░█░░░███░░░█           ░░███                                 
        ░   ░███  ░  █████ ████ ░███   ██████  ████████              
            ░███    ░░███ ░███  ░███  ███░░███░░███░░███             
            ░███     ░███ ░███  ░███ ░███████  ░███ ░░░              
            ░███     ░███ ░███  ░███ ░███░░░   ░███                  
            █████    ░░███████  █████░░██████  █████                 
           ░░░░░      ░░░░░███ ░░░░░  ░░░░░░  ░░░░░                  
                      ███ ░███                                       
                     ░░██████                                        
                      ░░░░░░                                         
  █████                                                              
 ░░███                                                               
  ░███         ██████  █████ ███ █████ ████████   ██████  █████ ████ 
  ░███        ███░░███░░███ ░███░░███ ░░███░░███ ███░░███░░███ ░███  
  ░███       ░███ ░███ ░███ ░███ ░███  ░███ ░░░ ░███████  ░███ ░███  
  ░███      █░███ ░███ ░░███████████   ░███     ░███░░░   ░███ ░███  
  ███████████░░██████   ░░████░████    █████    ░░██████  ░░███████  
 ░░░░░░░░░░░  ░░░░░░     ░░░░ ░░░░    ░░░░░      ░░░░░░    ░░░░░███  
                                                           ███ ░███  
                                                          ░░██████   
                                                           ░░░░░░    
                                                                     
                            "#
                        }
                        hr;
                        p { r#"
                            Hello! As you can probably tell, my name is Tyler Lowrey. I am a Software Engineer with a degree in Computer Science from Clemson University. I am interested in back-end web dev, robotics, and video game dev. Non-technical things I like are sci-fi books, playing video games, and hiking.
                            
                        "# }
                        #"social-links" {
                            a href="https://github.com/tylerlowrey" { p #"github-link" { img #"github-logo" src="images/github-mark-white.svg"; "My GitHub" } }
                            a href="https://linkedin.com/in/tylerlowrey" { p #"linkedin-link" { img #"linkedin-logo" src="images/In-White-128.png"; "My LinkedIn" } }
                        }
                    }
                }
            }
        }
    }.into_string()
}
