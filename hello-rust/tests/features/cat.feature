Feature: Cat feature
    
  Scenario: If we feed a satiated cat it will not become hungry
    Given a satiated cat
    When I feed the cat
    Then the cat is not hungry