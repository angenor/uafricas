<script setup lang="ts">
/**
 * Espace « Mes activités » (feature 013) : score de jeu, répartition,
 * historique des parties.
 *
 * Le score de jeu n'a rien à voir avec les points d'engagement : il a sa propre
 * page, à côté de « Mon engagement », et ne change pas le statut du membre.
 *
 * Tailwind v4 pur : aucune classe daisyUI (Principe VI).
 */
import type { HistoriquePartiesAPI, MonJeuAPI, PartieHistoriqueAPI } from '~/composables/useJeu'
import type { MesBadges } from '~/composables/useEngagement'

definePageMeta({ layout: false, middleware: 'auth' })

useHead({ title: 'Mes activités | AfricanS' })

const { monJeu, mesParties } = useJeu()
const { obtenirMesBadges } = useEngagement()

/**
 * Les distinctions du jeu sont des badges d'engagement à code `jeu_*` : le
 * moteur d'engagement les attribue, cette page n'en montre que la part ludique,
 * obtenus et à débloquer.
 */
const badges = ref<MesBadges | null>(null)
const distinctions = computed(() => ({
  obtenues: badges.value?.obtenus.filter(b => b.code.startsWith('jeu_')) ?? [],
  aDebloquer: badges.value?.a_debloquer.filter(b => b.code.startsWith('jeu_')) ?? [],
}))

const jeu = ref<MonJeuAPI | null>(null)
const historique = ref<HistoriquePartiesAPI | null>(null)
const page = ref(1)
const chargement = ref(true)
const erreur = ref('')

const TAILLE = 15
const totalPages = computed(() => Math.max(1, Math.ceil((historique.value?.total ?? 0) / TAILLE)))

const LIBELLES_CADRE: Record<PartieHistoriqueAPI['cadre'], string> = {
  libre: 'Partie',
  entrainement: 'Entraînement',
  defi: 'Défi',
  duel: 'Duel',
}

const origines = computed(() => [
  { libelle: 'Parties', score: jeu.value?.score_parties ?? 0 },
  { libelle: 'Défis', score: jeu.value?.score_defis ?? 0 },
  { libelle: 'Duels', score: jeu.value?.score_duels ?? 0 },
])

const libelleModule = (code: string | null) =>
  code ? (jeu.value?.par_module.find(m => m.code === code)?.libelle ?? code) : 'Tous modules'

const date = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })

const chargerHistorique = async () => {
  historique.value = await mesParties(page.value, TAILLE)
}

onMounted(async () => {
  try {
    const [j] = await Promise.all([monJeu(), chargerHistorique()])
    jeu.value = j
    // Accessoire : un échec ne masque pas la page.
    badges.value = await obtenirMesBadges().catch(() => null)
  }
  catch {
    erreur.value = 'Impossible de charger vos activités pour le moment.'
  }
  finally {
    chargement.value = false
  }
})

watch(page, () => {
  chargerHistorique().catch(() => { erreur.value = 'Impossible de charger l\'historique.' })
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Mes activités' }]"
      />
    </template>

    <div class="flex flex-col gap-6 pb-24">
      <header class="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes activités</h1>
          <p class="mt-1 text-[14px]/[1.5] text-af-corps">
            Votre score de jeu, votre place au Championship et l'historique de vos parties.
          </p>
        </div>
        <AfricansBouton icone="fa-solid fa-play" vers="/activites">Jouer</AfricansBouton>
      </header>

      <div v-if="chargement" class="flex flex-col gap-6">
        <div v-for="n in 2" :key="n" class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <p
        v-else-if="erreur || !jeu"
        class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur || 'Impossible de charger vos activités.' }}
      </p>

      <template v-else>
        <JeuResumeJoueur :jeu="jeu" />

        <div class="flex flex-wrap gap-3">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-ranking-star" vers="/activites/championship">
            Voir le Championship
          </AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-medal" vers="/mon-compte/engagement">
            Mes distinctions
          </AfricansBouton>
        </div>

        <!-- Distinctions du jeu -->
        <section
          v-if="distinctions.obtenues.length || distinctions.aDebloquer.length"
          class="rounded-[10px] border border-af-bordure bg-af-surface p-6"
        >
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Distinctions</h2>
          <p class="mt-1 text-[14px]/[1.5] text-af-corps">
            Elles ne rapportent pas de points : elles montrent ce que vous avez accompli au jeu, sur votre profil.
          </p>
          <div class="mt-4 flex flex-wrap gap-3">
            <EngagementBadgeSucces
              v-for="b in distinctions.obtenues"
              :key="b.code"
              :libelle="b.libelle"
              :description="b.description"
              :couleur="b.couleur"
              :icone="b.icone"
              obtenu
              :obtenu-at="b.obtenu_at"
            />
            <EngagementBadgeSucces
              v-for="b in distinctions.aDebloquer"
              :key="b.code"
              :libelle="b.libelle"
              :description="b.description"
              :couleur="b.couleur"
              :icone="b.icone"
              :obtenu="false"
            />
          </div>
        </section>

        <!-- État vide : rien joué encore -->
        <section
          v-if="jeu.score_total === 0 && (historique?.total ?? 0) === 0"
          class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center"
        >
          <font-awesome-icon icon="fa-solid fa-gamepad" class="text-4xl text-af-chocolat" />
          <h2 class="mt-4 text-[17px]/[1.4] font-bold text-af-encre">Vous n'avez pas encore joué</h2>
          <p class="mt-2 text-[14px]/[1.5] text-af-corps">
            Une partie dure quelques minutes. Chaque bonne réponse rapporte du score et fait monter votre pays.
          </p>
        </section>

        <template v-else>
          <section class="grid gap-6 lg:grid-cols-2">
            <div class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
              <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Par module</h2>
              <p v-if="jeu.par_module.length === 0" class="mt-3 text-[14px]/[1.5] text-af-atone">
                Aucun score de module encore.
              </p>
              <dl v-else class="mt-3 flex flex-col">
                <div
                  v-for="m in jeu.par_module"
                  :key="m.code"
                  class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3 first:border-t-0"
                >
                  <dt class="text-[14px]/[1.4] text-af-corps">{{ m.libelle }}</dt>
                  <dd class="text-[14px]/[1.4] font-bold tabular-nums text-af-encre">{{ m.score }}</dd>
                </div>
              </dl>
            </div>

            <div class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
              <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Par origine</h2>
              <dl class="mt-3 flex flex-col">
                <div
                  v-for="o in origines"
                  :key="o.libelle"
                  class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3 first:border-t-0"
                >
                  <dt class="text-[14px]/[1.4] text-af-corps">{{ o.libelle }}</dt>
                  <dd class="text-[14px]/[1.4] font-bold tabular-nums text-af-encre">{{ o.score }}</dd>
                </div>
              </dl>
              <p class="mt-2 text-[13px]/[1.5] text-af-atone">
                Les primes des défis et des duels ne sont rattachées à aucun module.
              </p>
            </div>
          </section>

          <section class="rounded-[10px] border border-af-bordure bg-af-surface">
            <h2 class="px-5 pt-5 pb-3 text-[17px]/[1.4] font-bold text-af-encre">
              Historique
              <span class="font-normal text-af-atone">({{ historique?.total ?? 0 }})</span>
            </h2>
            <ul class="divide-y divide-af-bordure">
              <li v-for="p in historique?.elements ?? []" :key="p.id">
                <NuxtLink
                  :to="`/activites/partie/${p.id}`"
                  class="flex flex-wrap items-center gap-x-4 gap-y-1 px-5 py-3 transition hover:bg-af-chocolat/[0.04]"
                >
                  <span class="w-28 shrink-0 text-[14px]/[1.4] font-bold text-af-encre">{{ LIBELLES_CADRE[p.cadre] }}</span>
                  <span class="min-w-0 flex-1 truncate text-[14px]/[1.4] text-af-corps">
                    {{ libelleModule(p.module_code) }}
                    <span v-if="p.etat === 'close'" class="text-af-atone">· interrompue</span>
                  </span>
                  <span class="text-[14px]/[1.4] tabular-nums text-af-corps">{{ p.bonnes }} / {{ p.nombre_epreuves }}</span>
                  <span class="w-12 text-right text-[14px]/[1.4] font-bold tabular-nums" :class="p.score_gagne > 0 ? 'text-af-vert' : 'text-af-atone'">
                    +{{ p.score_gagne }}
                  </span>
                  <span class="w-24 text-right text-[13px]/[1.4] text-af-atone">{{ date(p.created_at) }}</span>
                </NuxtLink>
              </li>
            </ul>
            <nav v-if="totalPages > 1" class="flex items-center justify-center gap-4 border-t border-af-bordure p-4" aria-label="Pages de l'historique">
              <AfricansBouton variante="secondaire" :desactive="page <= 1" @click="page -= 1">Précédent</AfricansBouton>
              <span class="text-[14px]/[1.4] text-af-corps">Page {{ page }} sur {{ totalPages }}</span>
              <AfricansBouton variante="secondaire" :desactive="page >= totalPages" @click="page += 1">Suivant</AfricansBouton>
            </nav>
          </section>
        </template>
      </template>
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>
