fn main() {
    // Basic templates.

    let handlebars = handlebars::Handlebars::new();
    let rendered = handlebars.render_template("{{id}}", &maplit::btreemap! {"id" => "value"});
    assert_eq!(rendered.unwrap(), "value");

    let handlebars = handlebars::Handlebars::new();
    let rendered = handlebars.render_template("{{id}}", &maplit::btreemap! {"ID" => "value"});
    assert_eq!(rendered.unwrap(), "");

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    let rendered = handlebars.render_template("{{id}}", &maplit::btreemap! {"ID" => "value"});
    assert!(rendered.is_err());

    // Template with condition.

    let template_with_condition =
        "is {{#if volume_id}}volume '{{volume_id}}'{{else}}filesystem '{{filesystem_id}}'{{/if}}";

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    let rendered = handlebars.render_template(
        template_with_condition,
        &maplit::btreemap! {"volume_id" => "volume_id", "filesystem_id" => "filesystem_id"},
    );
    assert_eq!(rendered.unwrap(), "is volume 'volume_id'");

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    let rendered = handlebars.render_template(
        template_with_condition,
        &maplit::btreemap! {"filesystem_id" => "filesystem_id"},
    );
    assert_eq!(rendered.unwrap(), "is filesystem 'filesystem_id'");

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    let rendered = handlebars
        .render_template(template_with_condition, &maplit::btreemap! {"svm_id" => "svm_id"});
    assert!(rendered.is_err());

    // Template with context.

    let template_with_context =
    "{{#with resource}}is {{#if volume_id}}volume '{{volume_id}}'{{else}}filesystem '{{filesystem_id}}'{{/if}}{{/with}}";

    let mut handlebars = handlebars::Handlebars::new();
    handlebars.set_strict_mode(true);
    let rendered = handlebars.render_template(
        template_with_context,
        &serde_json::json!({"resource": {"filesystem_id": "filesystem_id"}}),
    );
    assert_eq!(rendered.unwrap(), "is filesystem 'filesystem_id'");
}
