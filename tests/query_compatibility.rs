use efi_bank::BillingChargeListQuery;

#[test]
fn existing_charge_list_query_literal_and_serialization_remain_compatible() {
    let query = BillingChargeListQuery {
        charge_type: "billet".into(),
        begin_date: "2024-05-01".into(),
        end_date: "2024-05-30".into(),
        custom_id: None,
        limit: None,
        page: None,
        offset: None,
    };
    assert_eq!(
        serde_json::to_value(query).unwrap(),
        serde_json::json!({
            "charge_type": "billet",
            "begin_date": "2024-05-01",
            "end_date": "2024-05-30"
        })
    );
}
