#[derive(Clone)]
pub struct CharacterStats {
  pub str: CharacterStat,
  pub dex: CharacterStat,
  pub per: CharacterStat
}

impl CharacterStats {
  pub fn new(str: usize, dex: usize, per: usize) -> CharacterStats {
    CharacterStats {
      str: CharacterStat::new("Strength", str),
      dex: CharacterStat::new("Dexterity", dex),
      per: CharacterStat::new("Perception", per),
    }
  }

  pub fn as_vec(&self) -> Vec<&CharacterStat> {
    vec![&self.str, &self.dex, &self.per]
  }
}

#[derive(Clone)]
pub struct CharacterStat {
  name: String,
  level: usize,
  exp: usize,
}

impl CharacterStat {
  pub fn new(name: &str, level: usize) -> CharacterStat {
    CharacterStat {
      name: name.to_string(),
      level,
      exp: 0
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn lvl(&self) -> usize {
    self.level
  }

  pub fn current_exp(&self) -> usize {
    self.exp
  }

  pub fn exp_for_next_level(&self) -> usize {
    self.level * 10
  }

  pub fn gain_exp(&mut self, amount: usize) {
    self.exp += amount;
    self.check_level_up()
  }

  fn check_level_up(&mut self) {
    let required_exp = self.exp_for_next_level();
    if self.exp > required_exp {
      self.level = self.level + 1;
      self.exp = self.exp - required_exp;
    }
  }
}