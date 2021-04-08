pub use super::{ComplexType, Element, Kind, Occurence};

// use crate::{Field, ObjectImpl, TypeName};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Schema {
    #[serde(rename = "element", default)]
    pub elements: Vec<Element>,

    #[serde(rename = "simpleType", default)]
    pub simple_types: Vec<SimpleType>,

    #[serde(rename = "complexType", default)]
    pub complex_types: Vec<ComplexType>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct SimpleType {
    pub name: String,

    #[serde(rename = "restriction", default)]
    pub restrictions: Vec<Restriction>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct ComplexContent {
    mixed: bool,

    #[serde(rename = "extension", default)]
    pub extensions: Vec<Extension>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Extension {
    pub base: String,

    #[serde(rename = "sequence", default)]
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub base: String,

    #[serde(rename = "enumeration", default)]
    pub enumerations: Vec<Enumeration>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Enumeration {
    pub value: String,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Sequence {
    #[serde(rename = "element")]
    pub elements: Vec<Element>,
}

pub fn de<'de, T, R>(r: R) -> Result<T, serde_xml_rs::Error>
where
    R: std::io::Read,
    T: serde::Deserialize<'de>,
{
    let mut de = serde_xml_rs::Deserializer::new_from_reader(r).non_contiguous_seq_elements(true);

    T::deserialize(&mut de)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs as xml;

    #[test]
    fn parse_a_little_of_everything() {
        let s = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:import namespace="http://microsoft.com/wsdl/types/" />
  <xs:element name="StringToEnumConverter" nillable="true" type="StringToEnumConverter" />
  <xs:complexType name="StringToEnumConverter">
    <xs:complexContent mixed="false">
      <xs:extension base="JsonConverter" />
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="AccountingExportCreation" nillable="true" type="AccountingExportCreation" />
  <xs:complexType name="JsonConverter" abstract="true" />
  <xs:complexType name="AccountingExportCreation">
    <xs:sequence>
      <xs:element minOccurs="1" maxOccurs="1" name="CreditorPublicId" type="guid" />
      <xs:element minOccurs="1" maxOccurs="1" name="From" type="xs:dateTime" />
      <xs:element minOccurs="1" maxOccurs="1" name="To" type="xs:dateTime" />
      <xs:element minOccurs="1" maxOccurs="1" name="Format" type="AccountingExportFormatType" />
      <xs:element minOccurs="0" maxOccurs="1" name="BookKeepingTypesFilter" type="ArrayOfAccountingRecordType" />
      <xs:element minOccurs="1" maxOccurs="1" name="DateSelectionType" type="AccountingExportDateSelectionType" />
      <xs:element minOccurs="1" maxOccurs="1" name="SummarizeAccountsByDate" type="xs:boolean" />
    </xs:sequence>
  </xs:complexType>
  <xs:simpleType name="AccountingExportFormatType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Unknown" />
      <xs:enumeration value="SIE4" />
      <xs:enumeration value="CSV" />
    </xs:restriction>
  </xs:simpleType>
  <xs:complexType name="ArrayOfAccountingRecordType">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="unbounded" name="AccountingRecordType" type="AccountingRecordType" />
    </xs:sequence>
  </xs:complexType>
  <xs:simpleType name="AccountingRecordType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Unknown" />
      <xs:enumeration value="ProductSales" />
      <xs:enumeration value="ProductSalesWithReverseVAT" />
      <xs:enumeration value="RotRutDiscount" />
      <xs:enumeration value="PaymentToBankAccount" />
      <xs:enumeration value="OverPaymentToBankAccount" />
      <xs:enumeration value="CentRounding" />
      <xs:enumeration value="Interest" />
      <xs:enumeration value="ProductSalesEU" />
      <xs:enumeration value="ProductSalesEUVAT" />
      <xs:enumeration value="ProductSalesNonEU" />
      <xs:enumeration value="SupplierPaymentFromBankAccount" />
      <xs:enumeration value="SupplierPurchaseDebt" />
      <xs:enumeration value="SupplierPurchaseEU" />
      <xs:enumeration value="SupplierPurchaseEUVAT" />
      <xs:enumeration value="SupplierPurchaseNonEU" />
      <xs:enumeration value="CurrencyDifference" />
      <xs:enumeration value="FinanceCostNoRecourse" />
      <xs:enumeration value="SelfInvoiceDebt" />
      <xs:enumeration value="SelfInvoiceDebtVAT" />
      <xs:enumeration value="SelfInvoicePaymentFromBankAccount" />
      <xs:enumeration value="SelfInvoiceCreditation" />
      <xs:enumeration value="InvoiceSalesDebtRemoved" />
      <xs:enumeration value="WriteOff" />
      <xs:enumeration value="ReminderCostPayment" />
      <xs:enumeration value="Accrual" />
      <xs:enumeration value="AdminsitrationCost" />
      <xs:enumeration value="InvoiceSalesDebtAdded" />
      <xs:enumeration value="RestingVAT" />
      <xs:enumeration value="FreightCost" />
      <xs:enumeration value="OverPaymentDeleted" />
      <xs:enumeration value="UnmatchedPaymentToBankAccount" />
      <xs:enumeration value="UnmatchedPaymentDeleted" />
      <xs:enumeration value="FinanceCostWithRecourse" />
      <xs:enumeration value="ClientFundDebt" />
      <xs:enumeration value="NonPerformingLoanPurchase" />
      <xs:enumeration value="PurchasedNonPerformingLoanPayment" />
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"#;

        let _: Schema = super::de(s.as_bytes()).expect("Parsing");
    }

    #[test]
    fn parse_complexy_type() {
        let s = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
 <xs:complexType name="AccountingExportCreation">
    <xs:sequence>
      <xs:element minOccurs="1" maxOccurs="1" name="CreditorPublicId" type="guid" />
      <xs:element minOccurs="1" maxOccurs="1" name="From" type="xs:dateTime" />
      <xs:element minOccurs="1" maxOccurs="1" name="To" type="xs:dateTime" />
      <xs:element minOccurs="1" maxOccurs="1" name="Format" type="AccountingExportFormatType" />
      <xs:element minOccurs="0" maxOccurs="1" name="BookKeepingTypesFilter" type="ArrayOfAccountingRecordType" />
      <xs:element minOccurs="1" maxOccurs="1" name="DateSelectionType" type="AccountingExportDateSelectionType" />
      <xs:element minOccurs="1" maxOccurs="1" name="SummarizeAccountsByDate" type="xs:boolean" />
    </xs:sequence>
  </xs:complexType>
</xs:schema>
"#;

        let schema: Schema = xml::from_str(&s).expect("Parsing schema with simple type");
        assert_eq!(schema.complex_types.len(), 1);
        assert_eq!(schema.complex_types[0].sequences.len(), 1);
        let elements = &schema.complex_types[0].sequences[0].elements;
        assert_eq!(elements.len(), 7);
        assert_eq!(elements[0].min_occurs, Occurence::Bound(1));
        assert_eq!(elements[0].max_occurs, Occurence::Bound(1));
        assert_eq!(elements[0].name, "CreditorPublicId");
        assert_eq!(elements[0].kind, Some(Kind::Guid));
    }

    #[test]
    fn parse_compley_type_complex_content() {
        let xml = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">

 <xs:complexType name="EnumCompabilityDefault">
    <xs:complexContent mixed="false">
      <xs:extension base="Attribute">
        <xs:sequence>
          <xs:element minOccurs="1" maxOccurs="1" name="DefaultValue" type="xs:int" />
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>

</xs:schema>
"#;

        let schema: Schema = super::de(xml.as_bytes()).expect("Parsing");
        assert_eq!(schema.complex_types.len(), 1);
        assert_eq!(schema.complex_types[0].name, "EnumCompabilityDefault");
        assert_eq!(schema.complex_types[0].complex_contents.len(), 1);
        assert_eq!(schema.complex_types[0].complex_contents[0].mixed, false);
        assert_eq!(
            schema.complex_types[0].complex_contents[0].extensions.len(),
            1
        );
        let ext = &schema.complex_types[0].complex_contents[0].extensions[0];
        assert_eq!(ext.base, "Attribute");
        assert_eq!(ext.sequences.len(), 1);
        assert_eq!(ext.sequences[0].elements.len(), 1);
        assert_eq!(ext.sequences[0].elements[0].min_occurs, Occurence::Bound(1));
        assert_eq!(ext.sequences[0].elements[0].max_occurs, Occurence::Bound(1));
        assert_eq!(ext.sequences[0].elements[0].name, "DefaultValue");
        assert_eq!(ext.sequences[0].elements[0].kind, Some(Kind::Int));
    }

    #[test]
    fn parse_simple_type() {
        let s = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="AccountingExportFormatType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Unknown" />
      <xs:enumeration value="SIE4" />
      <xs:enumeration value="CSV" />
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"#;

        let schema: Schema = xml::from_str(&s).expect("Parsing schema with simple type");
        assert_eq!(schema.simple_types.len(), 1);
        assert_eq!(schema.simple_types[0].name, "AccountingExportFormatType");
        assert_eq!(schema.simple_types[0].restrictions.len(), 1);
        assert_eq!(schema.simple_types[0].restrictions[0].enumerations.len(), 3);
        let enums = &schema.simple_types[0].restrictions[0].enumerations;
        assert_eq!(enums[0].value, "Unknown");
        assert_eq!(enums[1].value, "SIE4");
        assert_eq!(enums[2].value, "CSV");
    }

    /// A test to make sure we use the unordered feature.
    #[test]
    fn parse_unordered() {
        #[derive(serde::Deserialize)]
        struct Bar;
        #[derive(serde::Deserialize)]
        struct Foo;
        #[derive(serde::Deserialize)]
        struct Thing {
            pub bar: Vec<Bar>,
            pub foo: Vec<Foo>,
        }

        let xml = "<thing><foo/> <bar/> <foo/> </thing>";
        let _: Thing = super::de(xml.as_bytes()).expect("Parsing");
    }

    #[test]
    fn parse_billecta_xsd() {
        let xsd = crate::BILLECTA_XSD;

        let schema: Schema = super::de(xsd.as_bytes()).expect("Parsing");

        for complex in &schema.complex_types {
            assert!(
                complex.complex_contents.len() < 2,
                "{} contains more than 1 complex contents",
                complex.name
            );

            assert!(
                complex.sequences.len() < 2,
                "{} contains more than one sequences",
                complex.name
            );

            // Assert that the ComplexTypes declaring arrays are typed.
            if complex.name.starts_with("ArrayOf") {
                assert_eq!(complex.sequences.len(), 1);
                assert_eq!(complex.sequences[0].elements.len(), 1);
            }
        }

        for elem in &schema.elements {
            match &elem.kind {
                Some(Kind::Object(_)) => (),
                Some(Kind::Array(inner)) if matches!(**inner, Kind::Object(_)) => (),
                None => (),
                Some(other) => {
                    assert!(
                        false,
                        "{} contains a root element of kind `{}`",
                        elem.name, other
                    );
                }
            }
        }

        for t in &schema.simple_types {
            assert!(
                t.restrictions.len() < 2,
                "{} contains more than 1 restrictions",
                t.name
            );

            if !t.restrictions.is_empty() {
                assert_eq!(t.restrictions[0].base, "xs:string");
            }
        }

        println!("Complex types: {}", schema.complex_types.len());
        println!("Simple types: {}", schema.simple_types.len());
        println!("Elements: {}", schema.elements.len());
    }
}
