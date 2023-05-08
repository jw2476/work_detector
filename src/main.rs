use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivitySegment {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    name: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duration {
    #[serde(rename="startTimestamp")]
    start: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceVisit {
    location: Location,
    duration: Duration
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TimelineEvent {
    #[serde(rename="activitySegment")]
    ActivitySegment(ActivitySegment),
    #[serde(rename="placeVisit")]
    PlaceVisit(PlaceVisit)
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename="timelineObjects")]
    timeline: Vec<TimelineEvent>
}

fn main() {
    std::env::args().skip(1).for_each(|file| {
        let data: Data = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        let work_days = data.timeline.iter().cloned().filter_map(|event| match event {
            TimelineEvent::PlaceVisit(visit) => Some(visit),
            _ => None
        }).filter(|visit| visit.location.name.is_some() && visit.location.name.as_ref().unwrap() == "Eight Bells Framing and Gallery").collect::<Vec<PlaceVisit>>();
        
        println!("===== {} =====", file);
        println!("Worked for {} days", work_days.len());
        work_days.iter().for_each(|day| {
            println!("{}", &day.duration.start[0..10]);
        })
    })
}
