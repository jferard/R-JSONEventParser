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

use r_json_event_parser::json_lexer::LexerToken::{IntValue, FloatValue, BeginArray, EndArray};
use r_json_event_parser::json_lexer::{JSONLexError, JSONLexConsumer, LexerToken, JSONLexer, ConsumeError};
use r_json_event_parser::byte_source::ByteSource;

struct PrintConsumer;

impl JSONLexConsumer for PrintConsumer {
    fn consume(&mut self, token: Result<LexerToken, JSONLexError>, _line: usize, _column: usize) -> Result<(), ConsumeError> {
        println!("{:?}", token);
        Ok(())
    }
}

struct AssertEqualsConsumer {
    tokens: Vec<Result<LexerToken, JSONLexError>>,
}

impl AssertEqualsConsumer {
    fn new() -> Self {
        return AssertEqualsConsumer { tokens: vec!() };
    }
}


impl JSONLexConsumer for AssertEqualsConsumer {
    fn consume(&mut self, token: Result<LexerToken, JSONLexError>, _line: usize, _column: usize) -> Result<(), ConsumeError> {
        self.tokens.push(token);
        Ok(())
    }
}

#[test]
fn lex_example1() {
    let path = "tests/files/example1.json";
    let expected_tokens = vec!(
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("glossary".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("title".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("example glossary".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossDiv".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("title".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("S".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossList".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("GlossEntry".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("ID".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SGML".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("SortAs".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SGML".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossTerm".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Standard Generalized Markup Language".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("Acronym".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SGML".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("Abbrev".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ISO 8879:1986".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossDef".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("para".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("A meta-markup language, used to create markup languages such as DocBook.".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossSeeAlso".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginArray),
        Ok(LexerToken::String("GML".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("XML".into())),
        Ok(LexerToken::EndArray),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("GlossSee".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("markup".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject));
    test_file(path, expected_tokens);
}

#[test]
fn lex_example2() {
    let path = "tests/files/example2.json";
    let expected_tokens = vec!(
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("menu".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("file".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("value".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("File".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("popup".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("menuitem".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginArray),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("value".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("New".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("onclick".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("CreateNewDoc()".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("value".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Open".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("onclick".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("OpenDoc()".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("value".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Close".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("onclick".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("CloseDoc()".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndArray),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject)
    );
    test_file(path, expected_tokens);
}

#[test]
fn lex_example3() {
    let path = "tests/files/example3.json";
    let expected_tokens = vec!(
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("widget".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("debug".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("on".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("window".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("title".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Sample Konfabulator Widget".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("main_window".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("width".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("500".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("height".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("500".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("image".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("src".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Images/Sun.png".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("sun1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("hOffset".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("250".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("vOffset".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("250".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("alignment".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("center".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("text".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("data".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Click Here".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("size".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("36".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("style".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("bold".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("text1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("hOffset".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("250".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("vOffset".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("100".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("alignment".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("center".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("onMouseUp".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("sun1.opacity = (sun1.opacity / 100) * 90;".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn lex_example4() {
    let path = "tests/files/example4.json";
    let expected_tokens = vec!(
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("web-app".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginArray),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet-name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofaxCDS".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-class".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.cds.CDSServlet".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("init-param".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("configGlossary:installationAt".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Philadelphia, PA".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("configGlossary:adminEmail".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ksm@pobox.com".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("configGlossary:poweredBy".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Cofax".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("configGlossary:poweredByIcon".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/images/cofax.gif".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("configGlossary:staticPath".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/content/static".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("templateProcessorClass".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.WysiwygTemplate".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("templateLoaderClass".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.FilesTemplateLoader".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("templatePath".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("templates".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("templateOverridePath".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("defaultListTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("listTemplate.htm".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("defaultFileTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("articleTemplate.htm".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("useJSP".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BooleanValue(false)),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("jspListTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("listTemplate.jsp".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("jspFileTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("articleTemplate.jsp".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePackageTagsTrack".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("200".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePackageTagsStore".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("200".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePackageTagsRefresh".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("60".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cacheTemplatesTrack".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("100".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cacheTemplatesStore".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("50".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cacheTemplatesRefresh".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("15".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePagesTrack".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("200".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePagesStore".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("100".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePagesRefresh".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("10".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cachePagesDirtyRead".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("10".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("searchEngineListTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("forSearchEnginesList.htm".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("searchEngineFileTemplate".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("forSearchEngines.htm".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("searchEngineRobotsDb".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("WEB-INF/robots.db".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("useDataStore".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BooleanValue(true)),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreClass".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.SqlDataStore".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("redirectionClass".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.SqlRedirection".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreName".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofax".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreDriver".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("com.microsoft.jdbc.sqlserver.SQLServerDriver".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreUrl".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("jdbc:microsoft:sqlserver://LOCALHOST:1433;DatabaseName=goon".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreUser".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("sa".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStorePassword".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("dataStoreTestQuery".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreTestQuery".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SET NOCOUNT ON;select test='test';".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreLogFile".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/usr/local/tomcat/logs/datastore.log".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreInitConns".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("10".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreMaxConns".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("100".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreConnUsageLimit".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("100".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataStoreLogLevel".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("debug".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("maxUrlLength".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("500".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet-name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofaxEmail".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-class".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.cds.EmailServlet".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("init-param".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("mailHost".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("mail1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("mailHostOverride".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("mail2".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet-name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofaxAdmin".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-class".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.cds.AdminServlet".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet-name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("fileServlet".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-class".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.cds.FileServlet".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("servlet-name".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofaxTools".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-class".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("org.cofax.cms.CofaxToolsServlet".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("init-param".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("templatePath".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("toolstemplates/".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("log".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("logLocation".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/usr/local/tomcat/logs/CofaxTools.log".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("logMaxSize".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataLog".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataLogLocation".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/usr/local/tomcat/logs/dataLog.log".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("dataLogMaxSize".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("removePageCache".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/content/admin/remove?cache=pages&id=".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("removeTemplateCache".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/content/admin/remove?cache=templates&id=".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("fileTransferFolder".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/usr/local/tomcat/webapps/content/fileTransferFolder".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("lookInContext".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("1".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("adminGroupID".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::IntValue("4".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("betaServer".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BooleanValue(true)),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndArray),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("servlet-mapping".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("cofaxCDS".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cofaxEmail".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/cofaxutil/aemail/*".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cofaxAdmin".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/admin/*".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("fileServlet".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/static/*".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("cofaxTools".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/tools/*".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("taglib".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("taglib-uri".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("cofax.tld".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("taglib-location".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("/WEB-INF/tlds/cofax.tld".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
    );
    test_file(path, expected_tokens);
}

#[test]
fn lex_example5() {
    let path = "tests/files/example5.json";
    let expected_tokens = vec!(
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("menu".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("header".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SVG Viewer".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("items".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::BeginArray),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Open".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("OpenNew".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Open New".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::NullValue),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ZoomIn".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Zoom In".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ZoomOut".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Zoom Out".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("OriginalView".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Original View".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::NullValue),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Quality".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Pause".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Mute".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::NullValue),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Find".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Find...".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("FindAgain".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Find Again".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Copy".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("CopyAgain".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Copy Again".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("CopySVG".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Copy SVG".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ViewSVG".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("View SVG".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("ViewSource".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("View Source".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("SaveAs".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Save As".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::NullValue),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("Help".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::BeginObject),
        Ok(LexerToken::String("id".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("About".into())),
        Ok(LexerToken::ValueSeparator),
        Ok(LexerToken::String("label".into())),
        Ok(LexerToken::NameSeparator),
        Ok(LexerToken::String("About Adobe CVG Viewer...".into())),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndArray),
        Ok(LexerToken::EndObject),
        Ok(LexerToken::EndObject),
    );
    test_file(path, expected_tokens);
}

fn test_file(path: &str, expected_tokens: Vec<Result<LexerToken, JSONLexError>>) {
    let f = fs::File::open(path).expect("no file found");
    test_read(f, expected_tokens);
}

fn test_read<R: Read>(read: R, expected_tokens: Vec<Result<LexerToken, JSONLexError>>) {
    let byte_source = ByteSource::new(read);
    let mut consumer = AssertEqualsConsumer::new();
    let mut lexer = JSONLexer::new(byte_source);
    let _ = lexer.lex(&mut consumer);
    assert_eq!(expected_tokens, consumer.tokens);
}

#[test]
fn test_end_numbers() {
    test_read("0".as_bytes(),
              vec!(Ok(IntValue("0".into()))));
    test_read("-0".as_bytes(),
              vec!(Ok(IntValue("0".into()))));
    test_read("-0e7".as_bytes(),
              vec!(Ok(FloatValue("-0e7".into()))));
    test_read("1".as_bytes(),
              vec!(Ok(IntValue("1".into()))));
    test_read("1.5".as_bytes(),
              vec!(Ok(FloatValue("1.5".into()))));
    test_read("1.52".as_bytes(),
              vec!(Ok(FloatValue("1.52".into()))));
    test_read("1.5e-2".as_bytes(),
              vec!(Ok(FloatValue("1.5e-2".into()))));
    test_read("1.5e-27".as_bytes(),
              vec!(Ok(FloatValue("1.5e-27".into()))));
    test_read("1.5e2".as_bytes(),
              vec!(Ok(FloatValue("1.5e2".into()))));
    test_read("1.5e27".as_bytes(),
              vec!(Ok(FloatValue("1.5e27".into()))));
    test_read("-1".as_bytes(),
              vec!(Ok(IntValue("-1".into()))));
}

#[test]
fn test_numbers() {
    test_read("0]".as_bytes(),
              vec!(Ok(IntValue("0".into())), Ok(EndArray)));
    test_read("-0]".as_bytes(),
              vec!(Ok(IntValue("0".into())), Ok(EndArray)));
    test_read("-0e7]".as_bytes(),
              vec!(Ok(FloatValue("-0e7".into())), Ok(EndArray)));
    test_read("1]".as_bytes(),
              vec!(Ok(IntValue("1".into())), Ok(EndArray)));
    test_read("1.5]".as_bytes(),
              vec!(Ok(FloatValue("1.5".into())), Ok(EndArray)));
    test_read("1.52]".as_bytes(),
              vec!(Ok(FloatValue("1.52".into())), Ok(EndArray)));
    test_read("1.5e-2]".as_bytes(),
              vec!(Ok(FloatValue("1.5e-2".into())), Ok(EndArray)));
    test_read("1.5e-27]".as_bytes(),
              vec!(Ok(FloatValue("1.5e-27".into())), Ok(EndArray)));
    test_read("1.5e2]".as_bytes(),
              vec!(Ok(FloatValue("1.5e2".into())), Ok(EndArray)));
    test_read("1.5e27]".as_bytes(),
              vec!(Ok(FloatValue("1.5e27".into())), Ok(EndArray)));
    test_read("-1]".as_bytes(),
              vec!(Ok(IntValue("-1".into())), Ok(EndArray)));
}

#[test]
fn test_unexpected_char() {
    test_read("*".as_bytes(), vec!(
        Err(JSONLexError { msg: "Unexpected char `*`".into(), line: 0, column: 1 })
    ));
    test_read("foo".as_bytes(), vec!(
        Err(JSONLexError { msg: "Expected word `alse`".into(), line: 0, column: 2 }),
        Err(JSONLexError { msg: "Unexpected char `o`".into(), line: 0, column: 3 }),
    ));
}

#[test]
fn test_wrong_number() {
    test_read("[01]".as_bytes(),
              vec!(
                  Ok(BeginArray), Ok(IntValue("0".into())),
                  Ok(IntValue("1".into())), Ok(EndArray),
              ),
    );
    test_read("[1.]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Err(JSONLexError { msg: "Missing decimals `1.`".into(), line: 0, column: 4 }),
                  Ok(EndArray)
              ),
    );
    test_read("[-]".as_bytes(),
               vec!(
                    Ok(BeginArray),
                    Err(JSONLexError { msg: "Expected a digit `]`".into(), line: 0, column: 3 }),
                    Ok(EndArray)
               )
    );
    test_read("[1.5e]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Err(JSONLexError { msg: "Missing exp `1.5e`".into(), line: 0, column: 6 }),
                  Ok(EndArray)
              )
    );
    test_read("[1e-]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Err(JSONLexError { msg: "Missing exp `1e-`".into(), line: 0, column: 5 }),
                  Ok(EndArray)
              )
    );
}

#[test]
fn test_end() {
    test_read("-".as_bytes(),
              vec!(Err(JSONLexError { msg: "Missing digits `-`".into(), line: 0, column: 1 })));
    test_read("0.".as_bytes(),
               vec!(Err(JSONLexError { msg: "Missing decimals `0.`".into(), line: 0, column: 2 })));
    test_read("1.5e".as_bytes(),
               vec!(Err(JSONLexError { msg: "Missing exp `1.5e`".into(), line: 0, column: 4 })));
    test_read("1.5e-".as_bytes(),
               vec!(Err(JSONLexError { msg: "Missing exp `1.5e-`".into(), line: 0, column: 5 })));
    test_read("\"foo".as_bytes(),
              vec!(Err(JSONLexError { msg: "Unfinished string `foo`".into(), line: 0, column: 4 })));
}

#[test]
fn test_unicode() {
    test_read("[\"\\ub9d0\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("말".into())),
                  Ok(EndArray))
    );
    test_read("[\"a\\ub9d0\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("a말".into())),
                  Ok(EndArray))
    );
    test_read("[\"\\ub9d0b\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("말b".into())),
                  Ok(EndArray))
    );
    test_read("[\"-\\uB9D0-\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("-말-".into())),
                  Ok(EndArray))
    );
}

#[test]
fn test_wrong_unicode() {
    test_read("[\"-\\uZ9D0-\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Err(JSONLexError { msg: "Unknown hex digit `Z`".into(), line: 0, column: 6 }),
                  Ok(LexerToken::String("-9D0-".into())),
                  Ok(EndArray))
    );
    test_read("[\"-\\uFDD0-\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("-\u{fdd0}-".into())),
                  Ok(EndArray))
    );
}

#[test]
fn test_escape() {
    test_read("[\"-\\\"-\\\\-\\b-\\f-\\n-\\r-\\t-\"]".as_bytes(),
              vec!(
                  Ok(BeginArray),
                  Ok(LexerToken::String("-\"-\\-\u{8}-\u{c}-\n-\r-\t-".into())),
                  Ok(EndArray))
    );
}