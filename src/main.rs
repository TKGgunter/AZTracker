//NOTE
//all tab related logic and styling were originally made by 
//https://www.w3schools.com/howto/howto_js_tabs.asp
//sort table
//https://www.w3schools.com/howto/howto_js_sort_table.asp
//https://www.geeksforgeeks.org/how-to-make-html-table-expand-on-click-using-javascript/
//

//TODO
//- Sort dates
//- Handle new years.
//- Robust implementation of default to current or closest month
//- make vertical threshold browser agnostic.
//- tags are not good. they should be handelled the same way leadership principles are.
//- const file?
//- handle toml parsing errors - this requires some altering of the toml crate
//- link opens for feedback opens in new tab or window or something https://www.thesitewizard.com/html-tutorial/open-links-in-new-window-or-tab.shtml
//- Details there is a preceeding new line. I would like to get rid of it.

use std::fs::File;
use std::io::prelude::*;

use serde_derive::Deserialize;
use toml;
use clap::{Command, Arg};
use colored::*;
use pulldown_cmark::{html, Options, Parser};

const UNWRAP_DATE_FAIL :&'static str = "In input file date was not properly formatted.";

const MONTHS : [&'static str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "June", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const LEADERSHIP : [&'static str; 16] = ["Customer Obsession", "Ownership", "Invent and Simplify", "Are Right, A Lot", "Learn and Be Curious", "Hire and Develop the Best", "Insist on the Highest Standards", "Think Big", "Bias for Action", "Frugality", "Earn Trust", "Dive Deep", "Have Backbone", "Deliver Results", "Strive to be Earth's Best Employer", "Success and Scale Bring Responsibility"];

const BR_COLOR : &'static str = "\"#1d6860\"";
const INVEST_IN_YOURSELF_COLOR : &'static str = "\"#555577\"";

const FEEDBACK_LINK : &'static str = "https://quip-amazon.com/yil8AxIlg78u/Accomplishment-and-Invest-in-Yourself-Tracker-Thoth#temp:C:LfXc7ddf386401743d0a4944584c";




fn generate_show_more_less_fn(rt: &mut String){
    *rt += "<script>
    $(\".show-more a\").on(\"click\", function() {
    var $this = $(this);
    var $content = $this.parent().prev(\"div.content\");
    var linkText = $this.text().toUpperCase();
    
    if(linkText === \"SHOW MORE\"){
        linkText = \"Show less\";
        $content.switchClass(\"hideContent\", \"showContent\", 50);
    } else {
        linkText = \"Show more\";
        $content.switchClass(\"showContent\", \"hideContent\", 50);
    };
    
    $this.text(linkText);
    });
</script>";
}

fn add_js_libraries(rt: &mut String){
    *rt += "<script src=
\"https://cdnjs.cloudflare.com/ajax/libs/jquery/3.3.1/jquery.min.js\">
    </script>\n";
    *rt += "<script src=\"http://code.jquery.com/ui/1.11.4/jquery-ui.js\"></script>\n";
}

fn begin_table(rt: &mut String){


    *rt += "<table =\"sortable\">\n";
    *rt +="<thread>";
    *rt += "\t<tr>\n";

    //TODO this should be done via a macro on the struct definition
    *rt += "\t\t<th class=\"smallColumn\">Date</th>\n";
    *rt += "\t\t<th style=\"width:22%\">Summary</th>\n";
    *rt += "\t\t<th>Details</th>\n";
    *rt += "\t\t<th style=\"width:20%\">Amazon Leadership Principles</th>\n";

    *rt += "\t</tr>\n";
    *rt += "</thread>\n";
    *rt += "<tbody>\n";

}
fn end_table(rt: &mut String){
    *rt += "</table>\n";
    *rt += "</tbody>\n";
    *rt += "</div>\n";
}
fn add_to_table(input: &Event, rt: &mut String){

    let tag = input.tags.as_ref().unwrap_or(&"".to_string()).to_lowercase();
    match tag.as_str() {
        "br" | "bar raising" => {
            *rt += &format!("\t<tr bgcolor={}>\n", BR_COLOR);
        },
        "invest" | "invest in your self" => {
            *rt += &format!("\t<tr bgcolor={}>\n", INVEST_IN_YOURSELF_COLOR);
        },
        "" => {
            *rt += "\t<tr>\n";
        }
        _=> {
            println!("{}: Tag is unknown {}.", "Error".red(), tag.yellow());
            *rt += "\t<tr>\n";
        }
    }


    let date = input.date.date.as_ref().expect(UNWRAP_DATE_FAIL);
    *rt += &format!("\t\t<td class=\"valueCells\">{}/{}/{}</td>\n", date.month, date.day, date.year);

    let summary = markdown_to_html(&input.summary);
    *rt += &format!("\t\t<td>{}</td>\n", summary);


    let details = markdown_to_html(input.details.as_ref().unwrap_or(&String::new()));
    *rt += &format!("\t\t<td> <div class=\"content hideContent\">{}</div>
\t\t<div class=\"show-more\">
\t\t    <a href=\"#\">Show more</a>
\t\t</div>
</td>\n", &details);

    let leadership_values = {
        let mut rt = String::new();
        for it in LEADERSHIP.iter(){
            if map_stringlp_to_eventlp(it, input) != 0{
                rt += it;
                rt += ", ";
            }
        } 
        rt
    };
    *rt += &format!("\t\t<td class=\"valueCells\">{}</td>\n", leadership_values);

    *rt += "\t</tr>\n";
}

fn generate_tab_links(input: &Vec<Event>, rt: &mut String){
    
    *rt += "\n<div class=\"tab\">";

    let mut month;
    let mut seen_month = 13;

    //////////
    //TODO
    //we should not literate over the full input list multiple times. 
    //We should keep the min and max months... we should think about this more deeply.
    let mut max_month = 13;
    for it in input.iter(){
        month  = it.date.date.as_ref().expect(UNWRAP_DATE_FAIL).month as usize;
        //TODO add to set.
        if max_month != month { //TODO if things are out of order we will have problems.
            max_month = month;
        }
    }
    //////////

    for (i, it) in input.iter().enumerate(){
        month  = it.date.date.as_ref().expect(UNWRAP_DATE_FAIL).month as usize;
        if seen_month != month { //TODO if things are out of order we will have problems.
            seen_month = month;
            let id_tag = if month == max_month {  "id=\"defaultOpen\"" } else { "" };

            *rt += &format!("<button class=\"tablinks\" onclick=\"openMonth(event, '{0}')\" {1}>{0}</button>\n", MONTHS[month-1], id_tag);
        }
    }

    *rt += "</div>\n";
}

fn generate_js_openmonth(rt: &mut String){
    *rt += "
<script>
function openMonth(evt, monthName) {
  // Declare all variables
  var i, tabcontent, tablinks;

  // Get all elements with class=\"tabcontent\" and hide them
  tabcontent = document.getElementsByClassName(\"tabcontent\");
  for (i = 0; i < tabcontent.length; i++) {
    tabcontent[i].style.display = \"none\";
  }

  // Get all elements with class=\"tablinks\" and remove the class \"active\"
  tablinks = document.getElementsByClassName(\"tablinks\");
  for (i = 0; i < tablinks.length; i++) {
    tablinks[i].className = tablinks[i].className.replace(\" active\", \"\");
  }

  // Show the current tab, and add an \"active\" class to the button that opened the tab
  document.getElementById(monthName).style.display = \"block\";
  evt.currentTarget.className += \" active\";
}
</script>
";
}
fn generate_css(rt: &mut String){
    *rt += "<style type=\"text/css\">";
    *rt += "\n/* Style the tab */
.tab {
  overflow: hidden;
  border: 1px solid #ccc;
  background-color: #f1f1f1;
}

/* Style the buttons that are used to open the tab content */
.tab button {
  font-size: 20px;
  background-color: inherit;
  float: left;
  border: none;
  outline: none;
  cursor: pointer;
  padding: 14px 16px;
  transition: 0.3s;
}

/* Change background color of buttons on hover */
.tab button:hover {
  background-color: #ddd;
}

/* Create an active/current tablink class */
.tab button.active {
  background-color: #ccc;
}

/* Style the tab content */
.tabcontent {
  display: none;
  padding: 6px 12px;
  border-top: none;
}

/* Style for event tag legend */
.legend {
    font-size: 1.1em;
    padding-left: 16px;
}

table {
  table-layout: fixed;
  width: 100%;
  border-collapse: collapse;
  border: 3px solid purple;
}

th {
  width: 30%;
  border-bottom: solid 1px #aaaaaa
}

th, td {
  padding: 7px;
  border-right: solid 1px #aaaaaa; 
  border-left: solid 1px #aaaaaa;
  font-size: 1.02em;
  line-height: 1.5;
}

td {
  border-bottom: solid 1px #aaaaaa;
}

.summary th, tr {
  padding: 7px;
  font-size: 1.02em;
  line-height: 1.5;
  border-left: 0px;
  border-right: 0px;
}

.hideContent {
    overflow: hidden;
    line-height: 1.3em;
    height: 5.05em;
}

.showContent {
    line-height: 1.3em;
    height: auto;
}
.smallColumn {
    width: 7%;
}
.valueCells{
   text-align: center;
}
a:link {
  color: pink;
}
a:visited {
  color: pink;
}
";
    *rt += "\n</style>";
}

fn map_stringlp_to_eventlp(stringlp: &str, eventlp: &Event)->u8{

    macro_rules! ref_unwrap_or {
        ($option:expr, $rt:literal)=>{
            *$option.as_ref().unwrap_or(&$rt)
        }

    }

    match stringlp {
        "Customer Obsession"  => {ref_unwrap_or!(eventlp.customer_obsession, 0)},
        "Ownership"           => {ref_unwrap_or!(eventlp.ownership, 0)},
        "Invent and Simplify" => {ref_unwrap_or!(eventlp.invent_and_simplify, 0)},
        "Are Right, A Lot"    => {ref_unwrap_or!(eventlp.are_right_alot, 0)},
        "Learn and Be Curious"=> {ref_unwrap_or!(eventlp.learn_and_be_curious,0)},
        "Hire and Develop the Best"      =>{ref_unwrap_or!(eventlp.hire_and_deleop_the_best, 0)},
        "Insist on the Highest Standards"=>{ref_unwrap_or!(eventlp.insist_on_the_highest_standards, 0)},
        "Think Big"         =>{ref_unwrap_or!(eventlp.think_big, 0)},
        "Bias for Action"   =>{ref_unwrap_or!(eventlp.bias_for_action, 0)},
        "Frugality"         =>{ref_unwrap_or!(eventlp.frugality, 0)},
        "Earn Trust"        =>{ref_unwrap_or!(eventlp.earn_trust, 0)},
        "Dive Deep"         =>{ref_unwrap_or!(eventlp.dive_deep, 0)},
        "Have Backbone" =>{ref_unwrap_or!(eventlp.have_backbone, 0)},
        "Deliver Results"                    =>{ref_unwrap_or!(eventlp.deliver_results, 0)},
        "Strive to be Earth's Best Employer" =>{ref_unwrap_or!(eventlp.strive_best_employer, 0)},
        "Success and Scale Bring Responsibility"=>{ref_unwrap_or!(eventlp.success_and_scale_brings_responsibility, 0)},
        _=>{panic!("Leadership string: {} cannot be found.", stringlp)}
    }
}

fn generate_leadership_principles_monthly_review(input: &Vec::<Event>, month: u8, rt: &mut String){

    let leadership_arr : [u8; 16] = {//Compute
        let mut arr = [0u8; 16];

        for it in input.iter(){
            if it.date.date.as_ref().expect(UNWRAP_DATE_FAIL).month != month {
                continue;
            }
            for (l, lt) in LEADERSHIP.iter().enumerate(){
                if map_stringlp_to_eventlp(lt, it) != 0 {
                    arr[l] += 1;
                }
            }
        }
        arr
    };

    *rt += "<table style=\"font-size:12px\">\n";
    *rt += "\t<tr>\n";
    *rt += "\t\t<th colspan=\"16\" style=\"font-size:150%\">Monthly Summary</th>\n";
    *rt += "\t</tr>\n";

    *rt += "\t<tr class=\"summary\">\n";
    for lt in LEADERSHIP.iter(){
        *rt += &format!("\t\t<th>{}</th>\n", lt);
    }
    *rt += "\t</tr>\n";

    *rt += "\t<tr class=\"summary\">";
    for lt in leadership_arr.iter(){
        match lt {
            1..=3 => {
                *rt += &format!("\t\t<th style=\"background-color:#7d5a0c\">{}</th>\n", *lt);
            },
            4..=u8::MAX => {
                *rt += &format!("\t\t<th style=\"background-color:#892e44\">{}</th>\n", *lt);
            },
            _=>{
                *rt += &format!("\t\t<th>{}</th>\n", *lt);
            }
        } 
    }
    *rt += "\t</tr>";

    *rt += "</table>";
    *rt += "</table>";
    
}

fn generate_report(input: &Report)->String{
    //TODO handle non existant reports and non existant event vec.
    //The type should prob be changed to a Option/Result that we get from toml.

    let mut rt = "<!DOCTYPE html>\n".to_string();
    rt += "<html>\n";
    let events = input.events.as_ref();


    rt += "<head>";
    add_js_libraries(&mut rt);
    generate_js_openmonth(&mut rt);
    generate_css(&mut rt);
    rt += "</head>";


    //TODO these colors should be handled in the style section.
    rt += "<body style=\"background-color:#3b3c3d; color:#ccc; font-family:Sans-Serif\">";
    generate_tab_links(events, &mut rt);


    //NOTE 
    //Generates Entry color key
    rt += "<p class=\"legend\">\n";
    rt += &format!("Bar raising moment - <font color={}> &#x2588; </font> </br>\n", BR_COLOR);
    rt += &format!("Invest in yourself - <font color={}> &#x2588; </font>\n", INVEST_IN_YOURSELF_COLOR);
    rt += "</p>\n";


    rt += "<p class=\"legend\">If you have feedback please follow this <a href=\"";
    rt += FEEDBACK_LINK;
    rt += "\">link</a>.</p>\n";


    let mut current_month;
    let mut page = String::new();
    let mut i = 0;


    //TODO if a poorly formatted date is submitted the error should contain bad date.
    while i < events.len() {
        current_month = events[i].date.date.as_ref().expect(UNWRAP_DATE_FAIL).month as usize;
        rt += &format!("\n<div id=\"{}\" class=\"tabcontent\">\n", MONTHS[current_month-1]);
    
        generate_leadership_principles_monthly_review(&events, current_month as u8, &mut page);

        begin_table(&mut page);
        add_to_table(&events[i], &mut page);


        //NOTE: loops through the remaining events and adds events in the same 
        //month to the table.
        let mut j = i+1;
        while  j < events.len(){
            let jth_month = events[j].date.date.as_ref().expect(UNWRAP_DATE_FAIL).month as usize;

            if jth_month >= 1 + current_month{
                end_table(&mut page);
                rt += &page;
                
                page = String::new();

                break;
            } else {
                add_to_table(&events[j], &mut page);
            }
            j+=1;
        }
        i = j;
    }
    end_table(&mut page);
    rt += &page;
    rt += "</body>";

    //The following ensures that "show more" is visible only when there are more than 3 lines of
    //text.
    rt += "<script>\n";
    rt +="
//TODO This is a pretty dumb way of doing things 
var tabs = document.getElementsByClassName(\"tablinks\");
for (var j = 0; j < tabs.length; j++) {
    tabs[j].click();
    var hide_content = document.getElementsByClassName(\"content hideContent\");
    var show_more = document.getElementsByClassName(\"show-more\");
    for (var i = 0; i < hide_content.length; i++) {

        _h = hide_content[i].scrollHeight;  
        offset_height = hide_content[i].offsetHeight;
        if (offset_height == _h && _h != 0) {
            show_more[i].style.display = \"none\";
        }
    }
}";
    rt += "document.getElementById(\"defaultOpen\").click();\n";
    rt += "</script>\n";
    generate_show_more_less_fn(&mut rt);

    rt += "</html>";
    return rt;
}


#[derive(Deserialize, Debug)]
struct Report{
    events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
struct Event{
    summary: String,
    details: Option<String>,
    date:    toml::value::Datetime,

    customer_obsession:   Option<u8>,
    ownership:            Option<u8>,
    invent_and_simplify:  Option<u8>,
    are_right_alot:       Option<u8>,
    learn_and_be_curious: Option<u8>,
    hire_and_deleop_the_best: Option<u8>,
    insist_on_the_highest_standards: Option<u8>,
    think_big:          Option<u8>,
    bias_for_action:    Option<u8>,
    frugality:          Option<u8>,
    earn_trust:         Option<u8>,
    dive_deep:          Option<u8>,
    have_backbone:      Option<u8>,
    deliver_results:    Option<u8>,
    strive_best_employer: Option<u8>,
    success_and_scale_brings_responsibility:  Option<u8>,

    tags: Option<String>,
}



fn markdown_to_html(input: &str)->String {
    //NOTE 
    //This is an example from
    //https://github.com/raphlinus/pulldown-cmark/blob/master/examples/string-to-string.rs

   
    
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);


    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    return html_output;
}


fn main() {
    let matches = Command::new("AZtracker")
        .version("0.1")
        .author("Thoth Gunter <mtgunter@amazon.com>")
        .about("Take a toml and convert it to html.")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .takes_value(true)
            .value_name("INPUT_FILE")
            .help("input file")
            )
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .takes_value(true)
            .value_name("OUTPUT_FILE")
            .help("output file")
            )
        .get_matches();

    

    let input_file_name =  matches.get_one::<String>("input");
    let input_file_text = match input_file_name {
        Some(file_name)=>{

            let mut f = File::open(file_name).expect("Could not open input file.");
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).expect("Could not read file to buffer.");
            buffer 
        },
        None=>{
            TEST.to_string()
        }
    };


    let report = {
        let rt : Report = toml::from_str(&input_file_text).expect("Could not parse toml input file.");
        generate_report(&rt)
    };



    let output_file_name = matches.get_one::<String>("output");
    match output_file_name {
        Some(file_name)=>{
            let mut f = File::create(file_name).expect("Could not create output file.");
            f.write_all(report.as_bytes()).expect("Could not write to output file.");
        },
        _=>{
            println!("{}", report);
        }
    }


}

const TEST : &'static str = "
[[events]]
summary = \"This is an example.\"
date = 1979-05-27T07:32:00-08:00
details = \"\"\" 
This document was created using a proprietary tool found here, https://github.com/TKGgunter/AZTracker. <b>Testing</b>.

### Header 3

*Wild boy*
\"\"\" 
ownership = 1
earn_trust = 1
dive_deep = 1

[[events]]
summary = \"Other work.\"
date = 1979-06-27T07:32:00-08:00
details = \"\"\" 
Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut od
\"\"\" 
deliver_results = 1
bias_for_action = 1
are_right_alot = 1
tags = \"Bar raising\"

[[events]]
summary = \"Some next work.\"
date = 1979-06-27T07:32:00-08:00
details = \"\"\" 
Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut od
\"\"\" 
deliver_results = 1
bias_for_action = 1
are_right_alot = 1
tags = \"invest\"

[[events]]
summary = \"Some more news.\"
date = 1979-06-27T07:32:00-08:00
details = \"\"\" 
Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut od. Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut od
\"\"\" 
deliver_results = 1
bias_for_action = 1
are_right_alot = 1

[[events]]
summary = \"Some more news.\"
date = 2022-09-04
details = \"\"\" 
Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut od
\"\"\" 
customer_obsession = 1
bias_for_action = 1
are_right_alot = 1
";

