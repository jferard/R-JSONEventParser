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
use r_json_event_parser::json_lexer::ConsumeError;
use r_json_event_parser::json_parser::{JSONParseConsumer, JSONParseError, JSONParser, ParserToken};
use r_json_event_parser::json_parser::ParserToken::{BeginArray, BeginFile, BeginObject, BooleanValue, EndArray, EndFile, EndObject, IntValue, Key, NullValue, StringValue};

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
        Ok(BeginFile),
        Ok(BeginObject),
        Ok(Key("glossary".into())),
        Ok(BeginObject),
        Ok(Key("title".into())),
        Ok(StringValue("example glossary".into())),
        Ok(Key("GlossDiv".into())),
        Ok(BeginObject),
        Ok(Key("title".into())),
        Ok(StringValue("S".into())),
        Ok(Key("GlossList".into())),
        Ok(BeginObject),
        Ok(Key("GlossEntry".into())),
        Ok(BeginObject),
        Ok(Key("ID".into())),
        Ok(StringValue("SGML".into())),
        Ok(Key("SortAs".into())),
        Ok(StringValue("SGML".into())),
        Ok(Key("GlossTerm".into())),
        Ok(StringValue("Standard Generalized Markup Language".into())),
        Ok(Key("Acronym".into())),
        Ok(StringValue("SGML".into())),
        Ok(Key("Abbrev".into())),
        Ok(StringValue("ISO 8879:1986".into())),
        Ok(Key("GlossDef".into())),
        Ok(BeginObject),
        Ok(Key("para".into())),
        Ok(StringValue("A meta-markup language, used to create markup languages such as DocBook.".into())),
        Ok(Key("GlossSeeAlso".into())),
        Ok(BeginArray),
        Ok(StringValue("GML".into())),
        Ok(StringValue("XML".into())),
        Ok(EndArray),
        Ok(EndObject),
        Ok(Key("GlossSee".into())),
        Ok(StringValue("markup".into())),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndFile),
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
        Ok(BeginFile),
        Ok(BeginObject),
        Ok(Key("menu".into())),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("file".into())),
        Ok(Key("value".into())),
        Ok(StringValue("File".into())),
        Ok(Key("popup".into())),
        Ok(BeginObject),
        Ok(Key("menuitem".into())),
        Ok(BeginArray),
        Ok(BeginObject),
        Ok(Key("value".into())),
        Ok(StringValue("New".into())),
        Ok(Key("onclick".into())),
        Ok(StringValue("CreateNewDoc()".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("value".into())),
        Ok(StringValue("Open".into())),
        Ok(Key("onclick".into())),
        Ok(StringValue("OpenDoc()".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("value".into())),
        Ok(StringValue("Close".into())),
        Ok(Key("onclick".into())),
        Ok(StringValue("CloseDoc()".into())),
        Ok(EndObject),
        Ok(EndArray),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndFile),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example3() {
    let path = "tests/files/example3.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(BeginFile),
        Ok(BeginObject),
        Ok(Key("widget".into())),
        Ok(BeginObject),
        Ok(Key("debug".into())),
        Ok(StringValue("on".into())),
        Ok(Key("window".into())),
        Ok(BeginObject),
        Ok(Key("title".into())),
        Ok(StringValue("Sample Konfabulator Widget".into())),
        Ok(Key("name".into())),
        Ok(StringValue("main_window".into())),
        Ok(Key("width".into())),
        Ok(IntValue("500".into())),
        Ok(Key("height".into())),
        Ok(IntValue("500".into())),
        Ok(EndObject),
        Ok(Key("image".into())),
        Ok(BeginObject),
        Ok(Key("src".into())),
        Ok(StringValue("Images/Sun.png".into())),
        Ok(Key("name".into())),
        Ok(StringValue("sun1".into())),
        Ok(Key("hOffset".into())),
        Ok(IntValue("250".into())),
        Ok(Key("vOffset".into())),
        Ok(IntValue("250".into())),
        Ok(Key("alignment".into())),
        Ok(StringValue("center".into())),
        Ok(EndObject),
        Ok(Key("text".into())),
        Ok(BeginObject),
        Ok(Key("data".into())),
        Ok(StringValue("Click Here".into())),
        Ok(Key("size".into())),
        Ok(IntValue("36".into())),
        Ok(Key("style".into())),
        Ok(StringValue("bold".into())),
        Ok(Key("name".into())),
        Ok(StringValue("text1".into())),
        Ok(Key("hOffset".into())),
        Ok(IntValue("250".into())),
        Ok(Key("vOffset".into())),
        Ok(IntValue("100".into())),
        Ok(Key("alignment".into())),
        Ok(StringValue("center".into())),
        Ok(Key("onMouseUp".into())),
        Ok(StringValue("sun1.opacity = (sun1.opacity / 100) * 90;".into())),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndFile),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example4() {
    let path = "tests/files/example4.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(BeginFile),
        Ok(BeginObject),
        Ok(Key("web-app".into())),
        Ok(BeginObject),
        Ok(Key("servlet".into())),
        Ok(BeginArray),
        Ok(BeginObject),
        Ok(Key("servlet-name".into())),
        Ok(StringValue("cofaxCDS".into())),
        Ok(Key("servlet-class".into())),
        Ok(StringValue("org.cofax.cds.CDSServlet".into())),
        Ok(Key("init-param".into())),
        Ok(BeginObject),
        Ok(Key("configGlossary:installationAt".into())),
        Ok(StringValue("Philadelphia, PA".into())),
        Ok(Key("configGlossary:adminEmail".into())),
        Ok(StringValue("ksm@pobox.com".into())),
        Ok(Key("configGlossary:poweredBy".into())),
        Ok(StringValue("Cofax".into())),
        Ok(Key("configGlossary:poweredByIcon".into())),
        Ok(StringValue("/images/cofax.gif".into())),
        Ok(Key("configGlossary:staticPath".into())),
        Ok(StringValue("/content/static".into())),
        Ok(Key("templateProcessorClass".into())),
        Ok(StringValue("org.cofax.WysiwygTemplate".into())),
        Ok(Key("templateLoaderClass".into())),
        Ok(StringValue("org.cofax.FilesTemplateLoader".into())),
        Ok(Key("templatePath".into())),
        Ok(StringValue("templates".into())),
        Ok(Key("templateOverridePath".into())),
        Ok(StringValue("".into())),
        Ok(Key("defaultListTemplate".into())),
        Ok(StringValue("listTemplate.htm".into())),
        Ok(Key("defaultFileTemplate".into())),
        Ok(StringValue("articleTemplate.htm".into())),
        Ok(Key("useJSP".into())),
        Ok(BooleanValue(false)),
        Ok(Key("jspListTemplate".into())),
        Ok(StringValue("listTemplate.jsp".into())),
        Ok(Key("jspFileTemplate".into())),
        Ok(StringValue("articleTemplate.jsp".into())),
        Ok(Key("cachePackageTagsTrack".into())),
        Ok(IntValue("200".into())),
        Ok(Key("cachePackageTagsStore".into())),
        Ok(IntValue("200".into())),
        Ok(Key("cachePackageTagsRefresh".into())),
        Ok(IntValue("60".into())),
        Ok(Key("cacheTemplatesTrack".into())),
        Ok(IntValue("100".into())),
        Ok(Key("cacheTemplatesStore".into())),
        Ok(IntValue("50".into())),
        Ok(Key("cacheTemplatesRefresh".into())),
        Ok(IntValue("15".into())),
        Ok(Key("cachePagesTrack".into())),
        Ok(IntValue("200".into())),
        Ok(Key("cachePagesStore".into())),
        Ok(IntValue("100".into())),
        Ok(Key("cachePagesRefresh".into())),
        Ok(IntValue("10".into())),
        Ok(Key("cachePagesDirtyRead".into())),
        Ok(IntValue("10".into())),
        Ok(Key("searchEngineListTemplate".into())),
        Ok(StringValue("forSearchEnginesList.htm".into())),
        Ok(Key("searchEngineFileTemplate".into())),
        Ok(StringValue("forSearchEngines.htm".into())),
        Ok(Key("searchEngineRobotsDb".into())),
        Ok(StringValue("WEB-INF/robots.db".into())),
        Ok(Key("useDataStore".into())),
        Ok(BooleanValue(true)),
        Ok(Key("dataStoreClass".into())),
        Ok(StringValue("org.cofax.SqlDataStore".into())),
        Ok(Key("redirectionClass".into())),
        Ok(StringValue("org.cofax.SqlRedirection".into())),
        Ok(Key("dataStoreName".into())),
        Ok(StringValue("cofax".into())),
        Ok(Key("dataStoreDriver".into())),
        Ok(StringValue("com.microsoft.jdbc.sqlserver.SQLServerDriver".into())),
        Ok(Key("dataStoreUrl".into())),
        Ok(StringValue("jdbc:microsoft:sqlserver://LOCALHOST:1433;DatabaseName=goon".into())),
        Ok(Key("dataStoreUser".into())),
        Ok(StringValue("sa".into())),
        Ok(Key("dataStorePassword".into())),
        Ok(StringValue("dataStoreTestQuery".into())),
        Ok(Key("dataStoreTestQuery".into())),
        Ok(StringValue("SET NOCOUNT ON;select test='test';".into())),
        Ok(Key("dataStoreLogFile".into())),
        Ok(StringValue("/usr/local/tomcat/logs/datastore.log".into())),
        Ok(Key("dataStoreInitConns".into())),
        Ok(IntValue("10".into())),
        Ok(Key("dataStoreMaxConns".into())),
        Ok(IntValue("100".into())),
        Ok(Key("dataStoreConnUsageLimit".into())),
        Ok(IntValue("100".into())),
        Ok(Key("dataStoreLogLevel".into())),
        Ok(StringValue("debug".into())),
        Ok(Key("maxUrlLength".into())),
        Ok(IntValue("500".into())),
        Ok(EndObject),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("servlet-name".into())),
        Ok(StringValue("cofaxEmail".into())),
        Ok(Key("servlet-class".into())),
        Ok(StringValue("org.cofax.cds.EmailServlet".into())),
        Ok(Key("init-param".into())),
        Ok(BeginObject),
        Ok(Key("mailHost".into())),
        Ok(StringValue("mail1".into())),
        Ok(Key("mailHostOverride".into())),
        Ok(StringValue("mail2".into())),
        Ok(EndObject),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("servlet-name".into())),
        Ok(StringValue("cofaxAdmin".into())),
        Ok(Key("servlet-class".into())),
        Ok(StringValue("org.cofax.cds.AdminServlet".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("servlet-name".into())),
        Ok(StringValue("fileServlet".into())),
        Ok(Key("servlet-class".into())),
        Ok(StringValue("org.cofax.cds.FileServlet".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("servlet-name".into())),
        Ok(StringValue("cofaxTools".into())),
        Ok(Key("servlet-class".into())),
        Ok(StringValue("org.cofax.cms.CofaxToolsServlet".into())),
        Ok(Key("init-param".into())),
        Ok(BeginObject),
        Ok(Key("templatePath".into())),
        Ok(StringValue("toolstemplates/".into())),
        Ok(Key("log".into())),
        Ok(IntValue("1".into())),
        Ok(Key("logLocation".into())),
        Ok(StringValue("/usr/local/tomcat/logs/CofaxTools.log".into())),
        Ok(Key("logMaxSize".into())),
        Ok(StringValue("".into())),
        Ok(Key("dataLog".into())),
        Ok(IntValue("1".into())),
        Ok(Key("dataLogLocation".into())),
        Ok(StringValue("/usr/local/tomcat/logs/dataLog.log".into())),
        Ok(Key("dataLogMaxSize".into())),
        Ok(StringValue("".into())),
        Ok(Key("removePageCache".into())),
        Ok(StringValue("/content/admin/remove?cache=pages&id=".into())),
        Ok(Key("removeTemplateCache".into())),
        Ok(StringValue("/content/admin/remove?cache=templates&id=".into())),
        Ok(Key("fileTransferFolder".into())),
        Ok(StringValue("/usr/local/tomcat/webapps/content/fileTransferFolder".into())),
        Ok(Key("lookInContext".into())),
        Ok(IntValue("1".into())),
        Ok(Key("adminGroupID".into())),
        Ok(IntValue("4".into())),
        Ok(Key("betaServer".into())),
        Ok(BooleanValue(true)),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndArray),
        Ok(Key("servlet-mapping".into())),
        Ok(BeginObject),
        Ok(Key("cofaxCDS".into())),
        Ok(StringValue("/".into())),
        Ok(Key("cofaxEmail".into())),
        Ok(StringValue("/cofaxutil/aemail/*".into())),
        Ok(Key("cofaxAdmin".into())),
        Ok(StringValue("/admin/*".into())),
        Ok(Key("fileServlet".into())),
        Ok(StringValue("/static/*".into())),
        Ok(Key("cofaxTools".into())),
        Ok(StringValue("/tools/*".into())),
        Ok(EndObject),
        Ok(Key("taglib".into())),
        Ok(BeginObject),
        Ok(Key("taglib-uri".into())),
        Ok(StringValue("cofax.tld".into())),
        Ok(Key("taglib-location".into())),
        Ok(StringValue("/WEB-INF/tlds/cofax.tld".into())),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndFile),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_example5() {
    let path = "tests/files/example5.json";
    let expected_tokens: Vec<Result<ParserToken, JSONParseError>> = vec!(
        Ok(BeginFile),
        Ok(BeginObject),
        Ok(Key("menu".into())),
        Ok(BeginObject),
        Ok(Key("header".into())),
        Ok(StringValue("SVG Viewer".into())),
        Ok(Key("items".into())),
        Ok(BeginArray),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Open".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("OpenNew".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Open New".into())),
        Ok(EndObject),
        Ok(NullValue),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("ZoomIn".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Zoom In".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("ZoomOut".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Zoom Out".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("OriginalView".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Original View".into())),
        Ok(EndObject),
        Ok(NullValue),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Quality".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Pause".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Mute".into())),
        Ok(EndObject),
        Ok(NullValue),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Find".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Find...".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("FindAgain".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Find Again".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Copy".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("CopyAgain".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Copy Again".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("CopySVG".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Copy SVG".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("ViewSVG".into())),
        Ok(Key("label".into())),
        Ok(StringValue("View SVG".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("ViewSource".into())),
        Ok(Key("label".into())),
        Ok(StringValue("View Source".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("SaveAs".into())),
        Ok(Key("label".into())),
        Ok(StringValue("Save As".into())),
        Ok(EndObject),
        Ok(NullValue),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("Help".into())),
        Ok(EndObject),
        Ok(BeginObject),
        Ok(Key("id".into())),
        Ok(StringValue("About".into())),
        Ok(Key("label".into())),
        Ok(StringValue("About Adobe CVG Viewer...".into())),
        Ok(EndObject),
        Ok(EndArray),
        Ok(EndObject),
        Ok(EndObject),
        Ok(EndFile),
    );
    test_file(path, expected_tokens);
}

#[test]
fn parse_wrong() {
    test_read("-foo".as_bytes(),
              vec!(
                  Ok(BeginFile),
                  Err(JSONParseError { msg: "Expected a digit `f`".into(), line: 0, column: 2 })
              ),
    );
    test_read("{\"foo\":-,\"bar\":10}".as_bytes(),
              vec!(
                  Ok(BeginFile),
                  Ok(BeginObject),
                  Ok(Key("foo".into())),
                  Err(JSONParseError { msg: "Expected a digit `,`".into(), line: 0, column: 9 })
              ),
    );
}

#[test]
fn test_unfinished() {
    test_read("{\"foo\":1".as_bytes(),
              vec!(
                  Ok(BeginFile),
                  Ok(BeginObject),
                  Ok(Key("foo".into())),
                  Ok(IntValue("1".into())),
                  Err(JSONParseError { msg: "Unexpected token `Ok(EndFile)`".into(),
                      line: 0, column: 8 }),
              ),
    );
}