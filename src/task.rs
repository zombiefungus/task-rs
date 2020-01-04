use std::collections::HashMap;

/// static std::string defaultProject;
// static default_project: String;
/// static std::string defaultDue
// static default_due: String,
/// static std::string defaultScheduled;
// static default_scheduled: String,
/// static bool searchCaseSensitive;
// static search_case_sensitive: bool,
/// static bool regex;
// static regex: bool,
/// static std::map <std::string, std::string> attributes;  // name -> type
// static attributes: HashMap<String, String>, // name -> type
/// static std::map <std::string, float> coefficients;
// static coefficients: HashMap<String, f32>,
/// static std::map <std::string, std::vector <std::string>> customOrder;
// static custom_order: HashMap<String, Vec<String>>,
/// static float urgencyProjectCoefficient;
// static urgency_project_coefficient: f32,
/// static float urgencyActiveCoefficient;
// static urgency_active_coefficient: f32,
/// static float urgencyScheduledCoefficient;
// static urgency_scheduled_coefficient: f32,
/// static float urgencyWaitingCoefficient;
// static urgency_waiting_coefficient: f32,
/// static float urgencyBlockedCoefficient;
// static urgency_blocked_coefficient: f32,
/// static float urgencyAnnotationsCoefficient;
// static urgency_annotations_coefficient: f32,
/// static float urgencyTagsCoefficient;
// static urgency_tags_coefficient: f32,
/// static float urgencyDueCoefficient;
// static urgency_due_coefficient: f32,
/// static float urgencyBlockingCoefficient;
// static urgency_blocking_coefficient: f32,
/// static float urgencyAgeCoefficient;
// static urgency_age_coefficient: f32,
/// static float urgencyAgeMax;
// static urgency_age_max: f32;

pub struct Task {
    /// std::map <std::string, std::string> data {};
    data: HashMap<String, String>,
    /// int id                                   {0};
    id: i32,
    /// float urgency_value                      {0.0};
    urgency_value: f32,
    /// bool recalc_urgency                      {true};
    recalc_urgency: bool,
    /// bool is_blocked                          {false};
    is_blocked: bool,
    /// bool is_blocking                         {false};
    is_blocking: bool,
    /// int annotation_count                     {0};
    annotation_count: i32,
}

/// enum status {pending, completed, deleted, recurring, waiting};
pub enum Status {
    Pending,
    Completed,
    Deleted,
    Recurring,
    Waiting,
}
/// enum dateState {dateNotDue, dateAfterToday, dateLaterToday, dateEarlierToday, dateBeforeToday};
pub enum DateState {
    DateNotDue,
    DateAfterToday,
    DateLaterToday,
    DateEarlierToday,
    DateBeforeToday,
}

impl Status {
    /// static status textToStatus (const std::string&);
    pub fn from_text(rep: &str) -> Self {
        todo!()
    }

    /// static std::string statusToText (status);
    pub fn to_string(&self) -> String {
        todo!()
    }
}

impl Task {
    /// Task () = default;
    pub fn new() -> Self {
        todo!()
    }

    /// Task (const std::string&);
    pub fn from_string(rep: &str) -> Self {
        todo!()
    }

    /// Task (const json::object*);
    pub fn from_json(rep: &str) -> Self {
        // TODO this is supposed to receive a "json::object*". check if can be
        // made with serialization
        todo!()
    }

    /// void parse (const std::string&);
    pub fn parse(&mut self, inp: &str) {
        // no idea what this does / should do
        todo!()
    }

    /// std::string composeF4 () const;
    pub fn compose_f4(&self) -> String {
        todo!()
    }

    /// std::string composeJSON (bool decorate = false) const;
    pub fn compose_json(&self, decorate: bool) -> String {
        // TODO check if this can be done using serialization
        todo!()
    }
    /// void setAsNow (const std::string&);
    pub fn set_as_now(&mut self, rep: &str) {
        todo!()
    }

    /// bool has (const std::string&) const;
    pub fn has(&self, pubsmt: &str) -> bool {
        todo!()
    }

    /// std::vector <std::string> all ();
    pub fn all(&self) -> Vec<String> {
        todo!()
    }

    /// const std::string identifier (bool shortened = false) const;
    pub fn identifier(&self, shortened: bool) -> String {
        todo!()
    }

    /// const std::string get (const std::string&) const;
    pub fn get(&self, smt: &str) -> String {
        todo!()
    }

    /// const std::string& get_ref (const std::string&) const;
    pub fn get_ref(&self, smt: &str) -> &str {
        todo!()
    }

    /// int get_int (const std::string&) const;
    pub fn get_int(&self, smt: &str) -> i32 {
        todo!()
    }

    /// unsigned long get_ulong (const std::string&) const;
    pub fn get_ulong(&self, smt: &str) -> u64 {
        todo!()
    }

    /// float get_float (const std::string&) const;
    pub fn get_float(&self, smt: &str) -> f32 {
        todo!()
    }

    /// time_t get_date (const std::string&) const;
    pub fn get_date(&self, smt: &str) -> i64 {
        todo!()
    }

    /// void set (const std::string&, const std::string&);
    pub fn set_str(&mut self, smt0: &str, smt1: &str) {
        todo!()
    }

    /// void set (const std::string&, int);
    pub fn set_int(&mut self, smt0: &str, smt1: i32) {
        todo!()
    }

    /// void remove (const std::string&);
    pub fn remove(&mut self, smt0: &str) {
        todo!()
    }

    /// bool is_ready () const;
    pub fn is_ready(&self) {
        todo!()
    }

    /// bool is_due () const;
    pub fn is_due(&self) -> bool {
        todo!()
    }

    /// bool is_dueyesterday () const;
    pub fn is_due_yesterday(&self) -> bool {
        todo!()
    }

    /// bool is_duetoday () const;
    pub fn is_due_today(&self) -> bool {
        todo!()
    }

    /// bool is_duetomorrow () const;
    pub fn is_due_tomorrow(&self) -> bool {
        todo!()
    }

    /// bool is_dueweek () const;
    pub fn is_due_week(&self) -> bool {
        todo!()
    }

    /// bool is_duemonth () const;
    pub fn is_due_month(&self) -> bool {
        todo!()
    }

    /// bool is_duequarter () const;
    pub fn is_due_quarter(&self) -> bool {
        todo!()
    }

    /// bool is_dueyear () const;
    pub fn is_due_year(&self) -> bool {
        todo!()
    }

    /// bool is_overdue () const;
    pub fn is_overdue(&self) -> bool {
        todo!()
    }

    /// bool is_udaPresent () const;
    pub fn is_uda_present(&self) -> bool {
        todo!()
    }

    /// bool is_orphanPresent () const;
    pub fn is_orphan_present(&self) -> bool {
        todo!()
    }

    /// status getStatus () const;
    pub fn get_status(&self) -> Status {
        todo!()
    }

    ///void setStatus (status);
    pub fn set_status(&mut self, status: Status) {
        todo!()
    }

    /// dateState getDateState (const std::string&) const;
    pub fn get_date_state(&self, smt: &str) -> DateState {
        todo!()
    }

    /// int getTagCount () const;
    pub fn get_tag_count(&self) -> i32 {
        todo!()
    }
    /// bool hasTag (const std::string&) const;
    pub fn has_tag(&self, tag: &str) -> bool {
        todo!()
    }
    /// void addTag (const std::string&);
    pub fn add_tag(&mut self, tag: &str) {
        todo!()
    }

    /// void addTags (const std::vector <std::string>&);
    pub fn add_tags(&mut self, tags: &Vec<String>) {
        todo!()
    }

    /// std::vector <std::string> getTags () const;
    pub fn get_tags(&self) -> Vec<String> {
        todo!()
    }

    /// void removeTag (const std::string&);
    pub fn remove_tag(&mut self, tag: &str) {
        todo!()
    }
    /// int getAnnotationCount () const;
    pub fn get_annotation_count(&self) -> i32 {
        todo!()
    }

    /// bool hasAnnotations () const;
    pub fn has_annotations(&self) -> bool {
        todo!()
    }

    /// std::map <std::string, std::string> getAnnotations () const;
    pub fn get_annotations(&self) -> HashMap<String, String> {
        todo!()
    }

    /// void setAnnotations (const std::map <std::string, std::string>&);
    pub fn set_annotations(&mut self, annotations: &HashMap<String, String>) {
        todo!()
    }

    /// void addAnnotation (const std::string&);
    pub fn add_annotation(&mut self, annotation: &str) {
        todo!()
    }

    /// void removeAnnotations ();
    pub fn remove_annotations(&mut self) {
        todo!()
    }

    //     void addDependency (int);
    //
    // void addDependency (const std::string&);
    // void removeDependency (int);
    // void removeDependency (const std::string&);
    // std::vector <int>         getDependencyIDs () const;
    // std::vector <std::string> getDependencyUUIDs () const;
    // std::vector <Task>        getDependencyTasks () const;
    //
    // std::vector <std::string> getUDAOrphanUUIDs () const;
    //
    // void substitute (const std::string&, const std::string&, const std::string&);
}

impl PartialEq for Task {
    /// bool operator== (const Task&);
    fn eq(&self, other: &Task) -> bool {
        todo!()
    }
}
