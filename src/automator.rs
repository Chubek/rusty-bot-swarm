use crate::utils::write_to_file;
use thirtyfour::prelude::*;
use tokio;
use crate::cookie::Cookie;
use std::path::Path;
use futures::executor::block_on;
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn automator() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444", caps).await?;


     driver.get("https://twitter.com").await?;
    
     Cookie::load_from_file_and_add(
         "/home/chubak/BotSwarmTwitter/PY/twitter.com.cookies.json", 
         &driver).await?;

     driver.get("https://twitter.com/search-advanced?lang=en").await?;

     sleep(Duration::from_millis(8000)).await;

     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;
     sleep(Duration::from_millis(100)).await;
     driver.execute_script("window.scrollTo(0, document.body.scrollHeight);").await?;

     sleep(Duration::from_millis(10000)).await;

     let page_links = driver.find_elements(
          By::XPath("//a[contains(@href, 'status')]")).await?;
     
     let mut rets = Vec::<String>::new();
     
     for x in page_links {
         match x.get_attribute("href").await? {
              Some(el) => rets.push(el),
              None => rets.push(String::new()),
         }
     }

     let joined = rets.join("\n");

     write_to_file("./links", joined).unwrap();

     



     
     // let elem = driver.find_element(
       //   By::XPath("//*[@data-testid = \"tweetTextarea_0\"]")).await?;
     
    // elem.send_keys("I love the smell napalm in the morning... Smells like #victory!!").await?;

   //  let elem_like = driver.find_element(
     //   By::XPath("//*[@aria-label = \"Like\"]")).await?;

    // elem_like.click().await?;
    
    // sleep(Duration::from_millis(10000)).await;

  //   let elem_rt = driver.find_element(
      //    By::XPath("//*[@data-testid = \"retweet\"]")).await?;
//
//     elem_rt.click().await?;

  //   sleep(Duration::from_millis(1000)).await;


    // let elem_btn = driver.find_element(
      //    By::XPath("//*[@data-testid = \"tweetButtonInline\"]")).await?;

    // let elem_rtt = driver.find_element(
    //      By::XPath("//*[@data-testid = \"retweetConfirm\"]")).await?;

  //   elem_rtt.click().await?;

     
  //   let elem_rtth = driver.find_element(
    //      By::XPath("//*[@href = \"/compose/tweet\"]")).await?;
//
   //  elem_rtth.click().await?;
//
  //   sleep(Duration::from_millis(1500)).await;

    // let elem_ta = driver.find_element(
     //     By::XPath("//*[@data-testid = \"tweetTextarea_0\"]")).await?;

    // elem_ta.click().await?;

   //  elem_ta.send_keys("Word!").await?;
//
   //  sleep(Duration::from_millis(1500)).await;

   //  let elem_btn = driver.find_element(
     //     By::XPath("//*[@data-testid = \"tweetButton\"]")).await?;

    // elem_btn.click().await?;
     

    // let elem_rep = driver.find_element(
    //      By::XPath("//*[@data-testid = \"reply\"]")).await?;
//

  //   elem_rep.click().await?;

  //   sleep(Duration::from_millis(2000)).await;

  //   let elem_ta = driver.find_element(
  //        By::XPath("//*[@data-testid = \"tweetTextarea_0\"]")).await?;
     
//     elem_ta.click().await?;
 //    elem_ta.send_keys("I").await?;
 //    sleep(Duration::from_millis(20)).await;

  //   elem_ta.send_keys("a").await?;
   //  sleep(Duration::from_millis(25)).await;
     
   //  elem_ta.send_keys("g").await?;
  //   sleep(Duration::from_millis(55)).await;

   //  elem_ta.send_keys("r").await?;
   //  sleep(Duration::from_millis(15)).await;

    // elem_ta.send_keys("e").await?;
    // sleep(Duration::from_millis(5)).await;

    // elem_ta.send_keys("e").await?;
    // sleep(Duration::from_millis(5)).await;
     
   //  elem_ta.click().await?;

   //  sleep(Duration::from_millis(5000)).await;

  //   let elem_btn = driver.find_element(
   //       By::XPath("//*[@data-testid = \"tweetButton\"]")).await?;

   //  elem_btn.click().await?;
     

    
     
     sleep(Duration::from_millis(5000)).await;
    
     driver.screenshot(Path::new("./sc.png")).await?;

     driver.quit().await?;

     Ok(())
}
