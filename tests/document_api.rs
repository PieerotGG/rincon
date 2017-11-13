
extern crate dotenv;
extern crate futures;
extern crate log4rs;
#[macro_use] extern crate serde_derive;
extern crate tokio_core;

extern crate arangodb_client;

mod test_fixture;

use test_fixture::*;
use arangodb_client::api::ErrorCode;
use arangodb_client::api::types::JsonString;
use arangodb_client::collection::CreateCollection;
use arangodb_client::connection::Error;
use arangodb_client::document::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Customer {
    name: String,
    contact: Vec<Contact>,
    gender: Gender,
    age: u16,
    active: bool,
    groups: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Contact {
    address: String,
    kind: ContactType,
    tag: Option<Tag>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Tag(String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum ContactType {
    Email,
    Phone,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct VipCustomer {
    name: String,
    contact: Vec<Contact>,
    age: u16,
    status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct CustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<String>>,
}

#[test]
fn insert_struct_document_without_key() {
    arango_user_db_test("test_document_user1", "test_document_db11", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer);
        let method = InsertDocument::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_without_key_and_return_new() {
    arango_user_db_test("test_document_user2", "test_document_db21", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer.clone());
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_struct_document_with_key() {
    arango_user_db_test("test_document_user3", "test_document_db31", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer)
            .with_key(DocumentKey::new("94711"));
        let method = InsertDocument::new("customers", new_document)
            .with_force_wait_for_sync(true);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94711", &document.id().to_string());
        assert_eq!("customers", document.id().collection_name());
        assert_eq!("94711", document.id().document_key());
        assert_eq!("94711", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
    });
}

#[test]
fn insert_struct_document_with_key_and_return_new() {
    arango_user_db_test("test_document_user4", "test_document_db41", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let new_document = NewDocument::from_content(customer.clone())
            .with_key(DocumentKey::new("94712"));
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers/94712", &document.id().to_string());
        assert_eq!("customers", document.id().collection_name());
        assert_eq!("94712", document.id().document_key());
        assert_eq!("94712", document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn insert_json_document_with_key_and_return_new() {
    arango_user_db_test("test_document_user5", "test_document_db51", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let json_doc = r#"{
            "name": "Jane Doe",
            "contact": [
                {
                    "address": "1-555-234523",
                    "kind": "Phone",
                    "tag": "work"
                }
            ],
            "gender": "Female",
            "age": 42,
            "active": true,
            "groups": []
        }"#;

        let new_document = NewDocument::from_content(JsonString::from_str(json_doc))
            .with_key(DocumentKey::new("7713996"));
        let method = InsertDocumentReturnNew::new("customers", new_document);
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert!(!document.id().document_key().is_empty());
        assert_eq!(document.id().document_key(), document.key().as_str());
        assert!(!document.revision().as_str().is_empty());
        assert!(document.content().as_str().starts_with(r#"{"_id":"customers/7713996","_key":"7713996","_rev":""#));
        assert!(document.content().as_str().ends_with(r#"","active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#));
    });
}

#[test]
fn insert_multiple_struct_documents_without_key() {
    arango_user_db_test("test_document_user6", "test_document_db61", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1);
        let new_document2 = NewDocument::from_content(customer2);
        let method = InsertDocuments::new("customers", vec![new_document1, new_document2])
            .with_force_wait_for_sync(true);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers", header1.id().collection_name());
            assert!(!header1.id().document_key().is_empty());
            assert_eq!(header1.id().document_key(), header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0));
        }

        if let Ok(ref header2) = documents.get(1).unwrap() {
            assert_eq!("customers", header2.id().collection_name());
            assert!(!header2.id().document_key().is_empty());
            assert_eq!(header2.id().document_key(), header2.key().as_str());
            assert!(!header2.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn insert_multiple_struct_documents_without_key_and_return_new() {
    arango_user_db_test("test_document_user7", "test_document_db71", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone());
        let new_document2 = NewDocument::from_content(customer2.clone());
        let method = InsertDocumentsReturnNew::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers", document1.id().collection_name());
            assert!(!document1.id().document_key().is_empty());
            assert_eq!(document1.id().document_key(), document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }
        if let Ok(ref document2) = documents.get(1).unwrap() {
            assert_eq!("customers", document2.id().collection_name());
            assert!(!document2.id().document_key().is_empty());
            assert_eq!(document2.id().document_key(), document2.key().as_str());
            assert!(!document2.revision().as_str().is_empty());
            assert_eq!(&customer2, document2.content());
        } else {
            panic!("Expected document 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn insert_multiple_struct_documents_with_key() {
    arango_user_db_test("test_document_user8", "test_document_db81", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1)
            .with_key(DocumentKey::new("94711"));
        let new_document2 = NewDocument::from_content(customer2)
            .with_key(DocumentKey::new("90815"));
        let method = InsertDocuments::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers/94711", &header1.id().to_string());
            assert_eq!("customers", header1.id().collection_name());
            assert_eq!("94711", header1.id().document_key());
            assert_eq!("94711", header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0))
        }

        if let Ok(ref header2) = documents.get(1).unwrap() {
            assert_eq!("customers/90815", &header2.id().to_string());
            assert_eq!("customers", header2.id().collection_name());
            assert_eq!("90815", header2.id().document_key());
            assert_eq!("90815", header2.key().as_str());
            assert!(!header2.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 2, but got: {:?}", documents.get(1))
        }
    });
}

#[test]
fn insert_multiple_struct_documents_with_key_and_return_new() {
    arango_user_db_test("test_document_user9", "test_document_db91", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone())
            .with_key(DocumentKey::new("94712"));
        let new_document2 = NewDocument::from_content(customer2.clone())
            .with_key(DocumentKey::new("90815"));
        let method = InsertDocumentsReturnNew::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers/94712", &document1.id().to_string());
            assert_eq!("customers", document1.id().collection_name());
            assert_eq!("94712", document1.id().document_key());
            assert_eq!("94712", document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }

        if let Ok(ref document2) = documents.get(1).unwrap() {
            assert_eq!("customers/90815", &document2.id().to_string());
            assert_eq!("customers", document2.id().collection_name());
            assert_eq!("90815", document2.id().document_key());
            assert_eq!("90815", document2.key().as_str());
            assert!(!document2.revision().as_str().is_empty());
            assert_eq!(&customer2, document2.content());
        } else {
            panic!("Expected document 2, but got: {:?}", documents.get(1));
        }
    });
}

#[test]
fn get_document_as_struct_inserted_as_struct() {
    arango_user_db_test("test_document_user10", "test_document_db101", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_struct_inserted_as_json_string() {
    arango_user_db_test("test_document_user11", "test_document_db111", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let json_doc = r#"{
            "name": "Jane Doe",
            "contact": [
                {
                    "address": "1-555-234523",
                    "kind": "Phone",
                    "tag": "work"
                }
            ],
            "gender": "Female",
            "age": 42,
            "active": true,
            "groups": []
        }"#;

        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(JsonString::new(json_doc))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_as_json_string_inserted_as_struct() {
    arango_user_db_test("test_document_user12", "test_document_db121", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7713996"))
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone());
        let document: Document<JsonString> = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        let expected = r#"{"active":true,"age":42,"contact":[{"address":"1-555-234523","kind":"Phone","tag":"work"}],"gender":"Female","groups":[],"name":"Jane Doe"}"#;
        assert_eq!(expected, document.content().as_str());
    });
}

#[test]
fn get_document_if_revision_matches() {
    arango_user_db_test("test_document_user13", "test_document_db131", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone())
            .with_if_match(revision.as_str().to_owned());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_if_revision_is_not_a_match() {
    arango_user_db_test("test_document_user14", "test_document_db141", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let method = GetDocument::new(document_id.clone())
            .with_if_non_match(String::from("not") + revision.as_str());
        let document = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", document.id().collection_name());
        assert_eq!(&document_id, document.id());
        assert_eq!(&document_key, document.key());
        assert_eq!(&revision, document.revision());
        assert_eq!(&customer, document.content());
    });
}

#[test]
fn get_document_but_revision_does_not_match() {
    arango_user_db_test("test_document_user15", "test_document_db151", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, _, revision) = header.deconstruct();

        let method = GetDocument::<Customer>::new(document_id)
            .with_if_match(String::from("not") + revision.as_str());
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn get_document_for_id_that_does_not_exist() {
    arango_user_db_test("test_document_user16", "test_document_db161", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (_, document_key, _) = header.deconstruct();

        let method = GetDocument::<Customer>::new(DocumentId::new("customers", "notexisting999"));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoDocumentNotFound, error.error_code());
                assert_eq!("document not found", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }

        let method = GetDocument::<Customer>::new(DocumentId::new("notexisting99", document_key.as_str()));
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(404, error.status_code());
                assert_eq!(ErrorCode::ArangoCollectionNotFound, error.error_code());
                assert_eq!("collection not found: notexisting99", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[ignore] //TODO refactor get document header to document exists (with possibly returning the revision)
#[test]
fn get_document_header() {
    arango_user_db_test("test_document_user20", "test_document_db201", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let inserted = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
                .with_key(DocumentKey::new("7721264"))
        ))).unwrap();

        let method = GetDocumentHeader::new(inserted.id().clone());
        let result = core.run(conn.execute(method)).unwrap();

        assert_eq!((), result);
    });
}

#[test]
fn replace_with_struct_document_without_revision() {
    arango_user_db_test("test_document_user30", "test_document_db301", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement);
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_ne!(&revision, updated.revision());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_revision() {
    arango_user_db_test("test_document_user31", "test_document_db311", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type() {
    arango_user_db_test("test_document_user32", "test_document_db321", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_old() {
    arango_user_db_test("test_document_user33", "test_document_db331", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_return_old(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(None, updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_new() {
    arango_user_db_test("test_document_user34", "test_document_db341", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_of_other_type_return_old_and_new() {
    arango_user_db_test("test_document_user35", "test_document_db351", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = VipCustomer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            age: 42,
            status: "active".to_owned(),
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(revision.clone());
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_return_new(true)
            .with_return_old(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_ignore_revisions_return_old_and_new() {
    arango_user_db_test("test_document_user36", "test_document_db361", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone())
            .with_revision(Revision::new("wrong_revision"));
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_ignore_revisions(true)
            .with_return_old(true)
            .with_return_new(true)
            .with_force_wait_for_sync(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_unknown_revision() {
    arango_user_db_test("test_document_user37", "test_document_db371", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, _) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement)
            .with_revision(Revision::new("wrong_revision"));
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_ignore_revisions(false)
            .with_return_old(true)
            .with_return_new(true)
            .with_force_wait_for_sync(true);
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn replace_with_struct_document_with_if_match_return_old_and_new() {
    arango_user_db_test("test_document_user38", "test_document_db381", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer.clone())
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement.clone());
        let method = ReplaceDocument::new(document_id.clone(), document_update)
            .with_if_match(revision.as_str().to_owned())
            .with_return_old(true)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(Some(&customer), updated.old_content());
        assert_eq!(Some(&replacement), updated.new_content());
    });
}

#[test]
fn replace_with_struct_document_with_if_match_unknown_revision() {
    arango_user_db_test("test_document_user39", "test_document_db391", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, _) = header.deconstruct();

        let replacement = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 42,
            active: true,
            groups: vec![],
        };

        let document_update = DocumentUpdate::new(document_key.clone(), replacement);
        let method = ReplaceDocument::<Customer, _>::new(document_id.clone(), document_update)
            .with_if_match("wrong_revision".to_owned())
            .with_return_old(true)
            .with_return_new(true);
        let result = core.run(conn.execute(method));

        match result {
            Err(Error::ApiError(error)) => {
                assert_eq!(412, error.status_code());
                assert_eq!(ErrorCode::ArangoConflict, error.error_code());
                assert_eq!("precondition failed", error.message());
            },
            _ => panic!("Error expected, but got: {:?}", &result),
        }
    });
}

#[test]
fn update_struct_document() {
    arango_user_db_test("test_document_user40", "test_document_db401", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: None,
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };
        let header = core.run(conn.execute(InsertDocument::new(
            "customers", NewDocument::from_content(customer)
        ))).unwrap();
        let (document_id, document_key, revision) = header.deconstruct();

        let update = CustomerUpdate {
            name: None,
            contact: Some(vec![
                Contact {
                    address: "1-555-8212494".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("mobile".to_owned())),
                }
            ]),
            gender: None,
            age: Some(43),
            active: None,
            groups: None,
        };

        let document_update = DocumentUpdate::new(document_key.clone(), update);
        let method = UpdateDocument::<_, Customer, Customer>::new(document_id.clone(), document_update)
            .with_return_new(true);
        let updated = core.run(conn.execute(method)).unwrap();

        assert_eq!("customers", updated.id().collection_name());
        assert_eq!(&document_id, updated.id());
        assert_eq!(&document_key, updated.key());
        assert!(!updated.revision().as_str().is_empty());
        assert_ne!(&revision, updated.revision());
        assert_eq!(&revision, updated.old_revision());
        assert_eq!(None, updated.old_content());
        let updated_content = updated.new_content().unwrap();
        assert_eq!("Jane Doe", &updated_content.name);
        assert_eq!(&Gender::Female, &updated_content.gender);
        assert_eq!(43, updated_content.age);
        assert_eq!(true, updated_content.active);
        assert_eq!(&Vec::<String>::new(), &updated_content.groups);
        let updated_contact = &updated_content.contact[0];
        assert_eq!("1-555-8212494", updated_contact.address);
        assert_eq!(&ContactType::Phone, &updated_contact.kind);
        assert_eq!(Some(&Tag("mobile".to_owned())), updated_contact.tag.as_ref());
    });
}

#[test]
fn insert_two_struct_documents_with_same_key() {
    arango_user_db_test("test_document_user50", "test_document_db501", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1)
            .with_key(DocumentKey::new("94711"));
        let new_document2 = NewDocument::from_content(customer2)
            .with_key(DocumentKey::new("94711"));
        let method = InsertDocuments::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref header1) = documents.get(0).unwrap() {
            assert_eq!("customers/94711", &header1.id().to_string());
            assert_eq!("customers", header1.id().collection_name());
            assert_eq!("94711", header1.id().document_key());
            assert_eq!("94711", header1.key().as_str());
            assert!(!header1.revision().as_str().is_empty());
        } else {
            panic!("Expected document header 1, but got: {:?}", documents.get(0))
        }

        if let Err(ref error) = documents.get(1).unwrap() {
            assert_eq!(ErrorCode::ArangoUniqueConstraintViolated, error.code());
            assert_eq!("unique constraint violated - in index 0 of type primary over [\"_key\"]", error.message());
        } else {
            panic!("Expected method error, but got: {:?}", documents.get(1))
        }
    });
}

#[test]
fn insert_two_struct_documents_with_same_key_and_return_new() {
    arango_user_db_test("test_document_user51", "test_document_db511", |conn, ref mut core| {
        core.run(conn.execute(CreateCollection::with_name("customers"))).unwrap();

        let customer1 = Customer {
            name: "Jane Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "1-555-234523".to_owned(),
                    kind: ContactType::Phone,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Female,
            age: 42,
            active: true,
            groups: vec![],
        };

        let customer2 = Customer {
            name: "John Doe".to_owned(),
            contact: vec![
                Contact {
                    address: "john.doe@mail.com".to_owned(),
                    kind: ContactType::Email,
                    tag: Some(Tag("work".to_owned())),
                }
            ],
            gender: Gender::Male,
            age: 27,
            active: true,
            groups: vec![],
        };

        let new_document1 = NewDocument::from_content(customer1.clone())
            .with_key(DocumentKey::new("94712"));
        let new_document2 = NewDocument::from_content(customer2.clone())
            .with_key(DocumentKey::new("94712"));
        let method = InsertDocumentsReturnNew::new("customers", vec![new_document1, new_document2]);
        let documents = core.run(conn.execute(method)).unwrap();

        if let Ok(ref document1) = documents.get(0).unwrap() {
            assert_eq!("customers/94712", &document1.id().to_string());
            assert_eq!("customers", document1.id().collection_name());
            assert_eq!("94712", document1.id().document_key());
            assert_eq!("94712", document1.key().as_str());
            assert!(!document1.revision().as_str().is_empty());
            assert_eq!(&customer1, document1.content());
        } else {
            panic!("Expected document 1, but got: {:?}", documents.get(0));
        }

        if let Err(ref error) = documents.get(1).unwrap() {
            assert_eq!(ErrorCode::ArangoUniqueConstraintViolated, error.code());
            assert_eq!("unique constraint violated - in index 0 of type primary over [\"_key\"]", error.message());
        } else {
            panic!("Expected method error, but got: {:?}", documents.get(1))
        }
    });
}
