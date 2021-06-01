/*
 * R-JSON Event Parser - a Rust JSON event based parser.
 *
 *    Copyright (C) 2021 J. Férard <https://github.com/jferard>
 *
 * This file is part of JSON Event Parser.
 *
 * R-JSON Event Parser is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * R-JSON Event Parser is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs;
use std::io::Read;

use r_json_event_parser::byte_source::ByteSource;
use r_json_event_parser::json_lexer::{ConsumeError};
use r_json_event_parser::json_parser::{JSONParseConsumer, JSONParseError, JSONParser, ParserToken};
use r_json_event_parser::json_parser::ParserToken::{BeginObject, Key};

struct AssertEqualsConsumer {
    tokens: Vec<Result<ParserToken, JSONParseError>>,
}

impl AssertEqualsConsumer {
    fn new() -> Self {
        return AssertEqualsConsumer { tokens: vec!() };
    }
}


impl JSONParseConsumer for AssertEqualsConsumer {
    fn consume(&mut self, token: Result<ParserToken, JSONParseError>) -> Result<(), ConsumeError> {
        self.tokens.push(token);
        Ok(())
    }
}

#[test]
fn parse_example1() {
    let path = "tests/files/example1.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("glossary".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("title".into())),
        Ok(ParserToken::StringValue("example glossary".into())),
        Ok(ParserToken::Key("GlossDiv".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("title".into())),
        Ok(ParserToken::StringValue("S".into())),
        Ok(ParserToken::Key("GlossList".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("GlossEntry".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("ID".into())),
        Ok(ParserToken::StringValue("SGML".into())),
        Ok(ParserToken::Key("SortAs".into())),
        Ok(ParserToken::StringValue("SGML".into())),
        Ok(ParserToken::Key("GlossTerm".into())),
        Ok(ParserToken::StringValue("Standard Generalized Markup Language".into())),
        Ok(ParserToken::Key("Acronym".into())),
        Ok(ParserToken::StringValue("SGML".into())),
        Ok(ParserToken::Key("Abbrev".into())),
        Ok(ParserToken::StringValue("ISO 8879:1986".into())),
        Ok(ParserToken::Key("GlossDef".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("para".into())),
        Ok(ParserToken::StringValue("A meta-markup language, used to create markup languages such as DocBook.".into())),
        Ok(ParserToken::Key("GlossSeeAlso".into())),
        Ok(ParserToken::BeginArray),
        Ok(ParserToken::StringValue("GML".into())),
        Ok(ParserToken::StringValue("XML".into())),
        Ok(ParserToken::EndArray),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::Key("GlossSee".into())),
        Ok(ParserToken::StringValue("markup".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject)
    );
    test_file(path, expected_tokens);
}

fn test_file(path: &str, expected_tokens: Vec<Result<ParserToken, JSONParseError>>) {
    let f = fs::File::open(path).expect("no file found");
    test_read(f, expected_tokens);
}

fn test_read<R: Read>(read: R, expected_tokens: Vec<Result<ParserToken, JSONParseError>>) {
    let byte_source = ByteSource::new(read);
    let mut consumer = AssertEqualsConsumer::new();
    let mut parser = JSONParser::new(byte_source);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected_tokens, consumer.tokens);
}

#[test]
fn parse_example2() {
    let path = "tests/files/example2.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("menu".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("file".into())),
        Ok(ParserToken::Key("value".into())),
        Ok(ParserToken::StringValue("File".into())),
        Ok(ParserToken::Key("popup".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("menuitem".into())),
        Ok(ParserToken::BeginArray),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("value".into())),
        Ok(ParserToken::StringValue("New".into())),
        Ok(ParserToken::Key("onclick".into())),
        Ok(ParserToken::StringValue("CreateNewDoc()".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("value".into())),
        Ok(ParserToken::StringValue("Open".into())),
        Ok(ParserToken::Key("onclick".into())),
        Ok(ParserToken::StringValue("OpenDoc()".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("value".into())),
        Ok(ParserToken::StringValue("Close".into())),
        Ok(ParserToken::Key("onclick".into())),
        Ok(ParserToken::StringValue("CloseDoc()".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndArray),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example3() {
    let path = "tests/files/example3.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("widget".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("debug".into())),
        Ok(ParserToken::StringValue("on".into())),
        Ok(ParserToken::Key("window".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("title".into())),
        Ok(ParserToken::StringValue("Sample Konfabulator Widget".into())),
        Ok(ParserToken::Key("name".into())),
        Ok(ParserToken::StringValue("main_window".into())),
        Ok(ParserToken::Key("width".into())),
        Ok(ParserToken::IntValue("500".into())),
        Ok(ParserToken::Key("height".into())),
        Ok(ParserToken::IntValue("500".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::Key("image".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("src".into())),
        Ok(ParserToken::StringValue("Images/Sun.png".into())),
        Ok(ParserToken::Key("name".into())),
        Ok(ParserToken::StringValue("sun1".into())),
        Ok(ParserToken::Key("hOffset".into())),
        Ok(ParserToken::IntValue("250".into())),
        Ok(ParserToken::Key("vOffset".into())),
        Ok(ParserToken::IntValue("250".into())),
        Ok(ParserToken::Key("alignment".into())),
        Ok(ParserToken::StringValue("center".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::Key("text".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("data".into())),
        Ok(ParserToken::StringValue("Click Here".into())),
        Ok(ParserToken::Key("size".into())),
        Ok(ParserToken::IntValue("36".into())),
        Ok(ParserToken::Key("style".into())),
        Ok(ParserToken::StringValue("bold".into())),
        Ok(ParserToken::Key("name".into())),
        Ok(ParserToken::StringValue("text1".into())),
        Ok(ParserToken::Key("hOffset".into())),
        Ok(ParserToken::IntValue("250".into())),
        Ok(ParserToken::Key("vOffset".into())),
        Ok(ParserToken::IntValue("100".into())),
        Ok(ParserToken::Key("alignment".into())),
        Ok(ParserToken::StringValue("center".into())),
        Ok(ParserToken::Key("onMouseUp".into())),
        Ok(ParserToken::StringValue("sun1.opacity = (sun1.opacity / 100) * 90;".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example4() {
    let path = "tests/files/example4.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("web-app".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet".into())),
        Ok(ParserToken::BeginArray),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet-name".into())),
        Ok(ParserToken::StringValue("cofaxCDS".into())),
        Ok(ParserToken::Key("servlet-class".into())),
        Ok(ParserToken::StringValue("org.cofax.cds.CDSServlet".into())),
        Ok(ParserToken::Key("init-param".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("configGlossary:installationAt".into())),
        Ok(ParserToken::StringValue("Philadelphia, PA".into())),
        Ok(ParserToken::Key("configGlossary:adminEmail".into())),
        Ok(ParserToken::StringValue("ksm@pobox.com".into())),
        Ok(ParserToken::Key("configGlossary:poweredBy".into())),
        Ok(ParserToken::StringValue("Cofax".into())),
        Ok(ParserToken::Key("configGlossary:poweredByIcon".into())),
        Ok(ParserToken::StringValue("/images/cofax.gif".into())),
        Ok(ParserToken::Key("configGlossary:staticPath".into())),
        Ok(ParserToken::StringValue("/content/static".into())),
        Ok(ParserToken::Key("templateProcessorClass".into())),
        Ok(ParserToken::StringValue("org.cofax.WysiwygTemplate".into())),
        Ok(ParserToken::Key("templateLoaderClass".into())),
        Ok(ParserToken::StringValue("org.cofax.FilesTemplateLoader".into())),
        Ok(ParserToken::Key("templatePath".into())),
        Ok(ParserToken::StringValue("templates".into())),
        Ok(ParserToken::Key("templateOverridePath".into())),
        Ok(ParserToken::StringValue("".into())),
        Ok(ParserToken::Key("defaultListTemplate".into())),
        Ok(ParserToken::StringValue("listTemplate.htm".into())),
        Ok(ParserToken::Key("defaultFileTemplate".into())),
        Ok(ParserToken::StringValue("articleTemplate.htm".into())),
        Ok(ParserToken::Key("useJSP".into())),
        Ok(ParserToken::BooleanValue(false)),
        Ok(ParserToken::Key("jspListTemplate".into())),
        Ok(ParserToken::StringValue("listTemplate.jsp".into())),
        Ok(ParserToken::Key("jspFileTemplate".into())),
        Ok(ParserToken::StringValue("articleTemplate.jsp".into())),
        Ok(ParserToken::Key("cachePackageTagsTrack".into())),
        Ok(ParserToken::IntValue("200".into())),
        Ok(ParserToken::Key("cachePackageTagsStore".into())),
        Ok(ParserToken::IntValue("200".into())),
        Ok(ParserToken::Key("cachePackageTagsRefresh".into())),
        Ok(ParserToken::IntValue("60".into())),
        Ok(ParserToken::Key("cacheTemplatesTrack".into())),
        Ok(ParserToken::IntValue("100".into())),
        Ok(ParserToken::Key("cacheTemplatesStore".into())),
        Ok(ParserToken::IntValue("50".into())),
        Ok(ParserToken::Key("cacheTemplatesRefresh".into())),
        Ok(ParserToken::IntValue("15".into())),
        Ok(ParserToken::Key("cachePagesTrack".into())),
        Ok(ParserToken::IntValue("200".into())),
        Ok(ParserToken::Key("cachePagesStore".into())),
        Ok(ParserToken::IntValue("100".into())),
        Ok(ParserToken::Key("cachePagesRefresh".into())),
        Ok(ParserToken::IntValue("10".into())),
        Ok(ParserToken::Key("cachePagesDirtyRead".into())),
        Ok(ParserToken::IntValue("10".into())),
        Ok(ParserToken::Key("searchEngineListTemplate".into())),
        Ok(ParserToken::StringValue("forSearchEnginesList.htm".into())),
        Ok(ParserToken::Key("searchEngineFileTemplate".into())),
        Ok(ParserToken::StringValue("forSearchEngines.htm".into())),
        Ok(ParserToken::Key("searchEngineRobotsDb".into())),
        Ok(ParserToken::StringValue("WEB-INF/robots.db".into())),
        Ok(ParserToken::Key("useDataStore".into())),
        Ok(ParserToken::BooleanValue(true)),
        Ok(ParserToken::Key("dataStoreClass".into())),
        Ok(ParserToken::StringValue("org.cofax.SqlDataStore".into())),
        Ok(ParserToken::Key("redirectionClass".into())),
        Ok(ParserToken::StringValue("org.cofax.SqlRedirection".into())),
        Ok(ParserToken::Key("dataStoreName".into())),
        Ok(ParserToken::StringValue("cofax".into())),
        Ok(ParserToken::Key("dataStoreDriver".into())),
        Ok(ParserToken::StringValue("com.microsoft.jdbc.sqlserver.SQLServerDriver".into())),
        Ok(ParserToken::Key("dataStoreUrl".into())),
        Ok(ParserToken::StringValue("jdbc:microsoft:sqlserver://LOCALHOST:1433;DatabaseName=goon".into())),
        Ok(ParserToken::Key("dataStoreUser".into())),
        Ok(ParserToken::StringValue("sa".into())),
        Ok(ParserToken::Key("dataStorePassword".into())),
        Ok(ParserToken::StringValue("dataStoreTestQuery".into())),
        Ok(ParserToken::Key("dataStoreTestQuery".into())),
        Ok(ParserToken::StringValue("SET NOCOUNT ON;select test='test';".into())),
        Ok(ParserToken::Key("dataStoreLogFile".into())),
        Ok(ParserToken::StringValue("/usr/local/tomcat/logs/datastore.log".into())),
        Ok(ParserToken::Key("dataStoreInitConns".into())),
        Ok(ParserToken::IntValue("10".into())),
        Ok(ParserToken::Key("dataStoreMaxConns".into())),
        Ok(ParserToken::IntValue("100".into())),
        Ok(ParserToken::Key("dataStoreConnUsageLimit".into())),
        Ok(ParserToken::IntValue("100".into())),
        Ok(ParserToken::Key("dataStoreLogLevel".into())),
        Ok(ParserToken::StringValue("debug".into())),
        Ok(ParserToken::Key("maxUrlLength".into())),
        Ok(ParserToken::IntValue("500".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet-name".into())),
        Ok(ParserToken::StringValue("cofaxEmail".into())),
        Ok(ParserToken::Key("servlet-class".into())),
        Ok(ParserToken::StringValue("org.cofax.cds.EmailServlet".into())),
        Ok(ParserToken::Key("init-param".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("mailHost".into())),
        Ok(ParserToken::StringValue("mail1".into())),
        Ok(ParserToken::Key("mailHostOverride".into())),
        Ok(ParserToken::StringValue("mail2".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet-name".into())),
        Ok(ParserToken::StringValue("cofaxAdmin".into())),
        Ok(ParserToken::Key("servlet-class".into())),
        Ok(ParserToken::StringValue("org.cofax.cds.AdminServlet".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet-name".into())),
        Ok(ParserToken::StringValue("fileServlet".into())),
        Ok(ParserToken::Key("servlet-class".into())),
        Ok(ParserToken::StringValue("org.cofax.cds.FileServlet".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("servlet-name".into())),
        Ok(ParserToken::StringValue("cofaxTools".into())),
        Ok(ParserToken::Key("servlet-class".into())),
        Ok(ParserToken::StringValue("org.cofax.cms.CofaxToolsServlet".into())),
        Ok(ParserToken::Key("init-param".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("templatePath".into())),
        Ok(ParserToken::StringValue("toolstemplates/".into())),
        Ok(ParserToken::Key("log".into())),
        Ok(ParserToken::IntValue("1".into())),
        Ok(ParserToken::Key("logLocation".into())),
        Ok(ParserToken::StringValue("/usr/local/tomcat/logs/CofaxTools.log".into())),
        Ok(ParserToken::Key("logMaxSize".into())),
        Ok(ParserToken::StringValue("".into())),
        Ok(ParserToken::Key("dataLog".into())),
        Ok(ParserToken::IntValue("1".into())),
        Ok(ParserToken::Key("dataLogLocation".into())),
        Ok(ParserToken::StringValue("/usr/local/tomcat/logs/dataLog.log".into())),
        Ok(ParserToken::Key("dataLogMaxSize".into())),
        Ok(ParserToken::StringValue("".into())),
        Ok(ParserToken::Key("removePageCache".into())),
        Ok(ParserToken::StringValue("/content/admin/remove?cache=pages&id=".into())),
        Ok(ParserToken::Key("removeTemplateCache".into())),
        Ok(ParserToken::StringValue("/content/admin/remove?cache=templates&id=".into())),
        Ok(ParserToken::Key("fileTransferFolder".into())),
        Ok(ParserToken::StringValue("/usr/local/tomcat/webapps/content/fileTransferFolder".into())),
        Ok(ParserToken::Key("lookInContext".into())),
        Ok(ParserToken::IntValue("1".into())),
        Ok(ParserToken::Key("adminGroupID".into())),
        Ok(ParserToken::IntValue("4".into())),
        Ok(ParserToken::Key("betaServer".into())),
        Ok(ParserToken::BooleanValue(true)),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndArray),
        Ok(ParserToken::Key("servlet-mapping".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("cofaxCDS".into())),
        Ok(ParserToken::StringValue("/".into())),
        Ok(ParserToken::Key("cofaxEmail".into())),
        Ok(ParserToken::StringValue("/cofaxutil/aemail/*".into())),
        Ok(ParserToken::Key("cofaxAdmin".into())),
        Ok(ParserToken::StringValue("/admin/*".into())),
        Ok(ParserToken::Key("fileServlet".into())),
        Ok(ParserToken::StringValue("/static/*".into())),
        Ok(ParserToken::Key("cofaxTools".into())),
        Ok(ParserToken::StringValue("/tools/*".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::Key("taglib".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("taglib-uri".into())),
        Ok(ParserToken::StringValue("cofax.tld".into())),
        Ok(ParserToken::Key("taglib-location".into())),
        Ok(ParserToken::StringValue("/WEB-INF/tlds/cofax.tld".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example5() {
    let path = "tests/files/example5.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("menu".into())),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("header".into())),
        Ok(ParserToken::StringValue("SVG Viewer".into())),
        Ok(ParserToken::Key("items".into())),
        Ok(ParserToken::BeginArray),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Open".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("OpenNew".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Open New".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::NullValue),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("ZoomIn".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Zoom In".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("ZoomOut".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Zoom Out".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("OriginalView".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Original View".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::NullValue),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Quality".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Pause".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Mute".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::NullValue),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Find".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Find...".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("FindAgain".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Find Again".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Copy".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("CopyAgain".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Copy Again".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("CopySVG".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Copy SVG".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("ViewSVG".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("View SVG".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("ViewSource".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("View Source".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("SaveAs".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("Save As".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::NullValue),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("Help".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::BeginObject),
        Ok(ParserToken::Key("id".into())),
        Ok(ParserToken::StringValue("About".into())),
        Ok(ParserToken::Key("label".into())),
        Ok(ParserToken::StringValue("About Adobe CVG Viewer...".into())),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndArray),
        Ok(ParserToken::EndObject),
        Ok(ParserToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_wrong() {
    test_read("-foo".as_bytes(),
              vec!(
                  Err(JSONParseError { msg: "Expected a digit `f`".into(), line: 0, column: 2 })
              )
    );
    test_read("{\"foo\":-,\"bar\":10}".as_bytes(),
              vec!(
                  Ok(BeginObject),
                  Ok(Key("foo".into())),
                  Err(JSONParseError { msg: "Expected a digit `,`".into(), line: 0, column: 9 })
              )
    );
}
