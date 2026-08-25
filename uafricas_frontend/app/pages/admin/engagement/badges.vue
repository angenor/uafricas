<script setup lang="ts">
/**
 * Back-office : badges et succès (daisyUI autorisé ici).
 *
 * Le formulaire de condition n'affiche que les champs de paramètres du
 * `type_condition` choisi : les 5 conditions n'utilisent pas les mêmes, et tout
 * afficher laisserait remplir des combinaisons que le serveur (et le CHECK SQL)
 * refuseraient.
 */
import { computed, ref, onMounted } from 'vue'
import {
  useAdminEngagement,
  type AdminBadge,
  type AdminCategorie,
  type AdminNiveau,
} from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  listerBadges, creerBadge, modifierBadge, supprimerBadge, attribuerBadge, retirerBadge,
  listerCategories, listerNiveaux, listerActionsDisponibles,
} = useAdminEngagement()

const badges = ref<AdminBadge[]>([])
const categories = ref<AdminCategorie[]>([])
const niveaux = ref<AdminNiveau[]>([])
const actions = ref<string[]>([])
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

/** Les 5 conditions automatiques, avec les paramètres que chacune consomme. */
const CONDITIONS = [
  { valeur: 'actions_comptees', libelle: 'Nombre d\'actions comptées', champs: ['action', 'seuil'] },
  { valeur: 'points_categorie', libelle: 'Points dans une catégorie', champs: ['categorie', 'seuil'] },
  { valeur: 'solde_total', libelle: 'Solde total de points', champs: ['seuil'] },
  { valeur: 'niveau_atteint', libelle: 'Niveau atteint', champs: ['niveau'] },
  { valeur: 'palier_popularite', libelle: 'Palier de popularité franchi', champs: ['seuil'] },
] as const

const COULEURS = ['green', 'chocolat', 'amber', 'sky', 'violet', 'rose', 'slate', 'gray']

const formulaire = ref(false)
const vierge = () => ({
  code: '', libelle: '', description: '',
  couleur: '' as string, icone: '' as string,
  manuel: false,
  type_condition: 'actions_comptees' as string,
  parametre_action: '' as string,
  parametre_categorie_id: '' as string,
  parametre_niveau_code: '' as string,
  seuil: null as number | null,
  ordre: 0,
  actif: true,
})
const nouveau = ref(vierge())

/** Champs de paramètres du type de condition sélectionné (formulaire de création). */
const champsCondition = computed<readonly string[]>(
  () => CONDITIONS.find(c => c.valeur === nouveau.value.type_condition)?.champs ?? [],
)

/** Idem pour une ligne du tableau (édition en place). */
const champsPour = (typeCondition: string | null): readonly string[] =>
  CONDITIONS.find(c => c.valeur === typeCondition)?.champs ?? []

const libelleCondition = (b: AdminBadge) => {
  if (b.manuel) return 'Attribution manuelle'
  return CONDITIONS.find(c => c.valeur === b.type_condition)?.libelle ?? b.type_condition ?? '-'
}

// Attribution / retrait manuels
const cible = ref<{ badge: AdminBadge | null, utilisateur_id: string, motif: string }>({
  badge: null, utilisateur_id: '', motif: '',
})

const rafraichir = async () => {
  chargement.value = true
  const [b, c, n, a] = await Promise.all([
    listerBadges(), listerCategories(), listerNiveaux(), listerActionsDisponibles(),
  ])
  badges.value = b
  categories.value = c
  niveaux.value = n
  actions.value = a.map(x => x.type_action)
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  erreur.value = ''
  setTimeout(() => { message.value = '' }, 3000)
}

const signaler = (e: unknown) => {
  message.value = ''
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || 'L\'opération a échoué.'
}

/** Vide les paramètres inutiles au type retenu : le serveur les refuserait sinon. */
const corpsBadge = (f: ReturnType<typeof vierge>) => ({
  code: f.code.trim(),
  libelle: f.libelle.trim(),
  description: f.description.trim(),
  couleur: f.couleur || null,
  icone: f.icone.trim() || null,
  manuel: f.manuel,
  type_condition: f.manuel ? null : f.type_condition,
  parametre_action: !f.manuel && f.parametre_action ? f.parametre_action : null,
  parametre_categorie_id: !f.manuel && f.parametre_categorie_id ? f.parametre_categorie_id : null,
  parametre_niveau_code: !f.manuel && f.parametre_niveau_code ? f.parametre_niveau_code : null,
  seuil: f.manuel ? null : f.seuil,
  ordre: f.ordre,
  actif: f.actif,
})

const ajouter = async () => {
  const f = nouveau.value
  if (!f.code.trim() || !f.libelle.trim() || !f.description.trim()) {
    erreur.value = 'Le code, le libellé et la description sont obligatoires.'
    return
  }
  try {
    await creerBadge(corpsBadge(f))
    formulaire.value = false
    nouveau.value = vierge()
    await rafraichir()
    notifier('Badge créé')
  } catch (e) { signaler(e) }
}

const enregistrer = async (b: AdminBadge) => {
  try {
    await modifierBadge(b.id, {
      libelle: b.libelle,
      description: b.description,
      couleur: b.couleur,
      icone: b.icone,
      manuel: b.manuel,
      type_condition: b.manuel ? null : b.type_condition,
      parametre_action: b.parametre_action,
      parametre_categorie_id: b.parametre_categorie_id,
      parametre_niveau_code: b.parametre_niveau_code,
      seuil: b.seuil,
      ordre: b.ordre,
      actif: b.actif,
    })
    notifier('Badge enregistré')
  } catch (e) { signaler(e) }
}

const supprimer = async (b: AdminBadge) => {
  if (b.nombre_detenteurs > 0) {
    erreur.value = `${b.nombre_detenteurs} membre(s) détiennent « ${b.libelle} » : décochez « Actif » pour le retirer du catalogue, ils doivent le conserver.`
    return
  }
  if (!confirm(`Supprimer définitivement le badge « ${b.libelle} » ?`)) return
  try {
    await supprimerBadge(b.id)
    await rafraichir()
    notifier('Badge supprimé')
  } catch (e) { signaler(e) }
}

const soumettreAttribution = async () => {
  const c = cible.value
  if (!c.badge || !c.utilisateur_id.trim()) {
    erreur.value = 'Indiquez l\'identifiant du membre.'
    return
  }
  try {
    const attribue = await attribuerBadge(c.badge.id, c.utilisateur_id.trim(), c.motif)
    await rafraichir()
    notifier(attribue
      ? `Badge « ${c.badge.libelle} » attribué, le membre a été notifié.`
      : 'Ce membre détenait déjà ce badge : rien n\'a changé, aucune notification envoyée.')
    cible.value = { badge: null, utilisateur_id: '', motif: '' }
  } catch (e) { signaler(e) }
}

const soumettreRetrait = async () => {
  const c = cible.value
  if (!c.badge || !c.utilisateur_id.trim()) {
    erreur.value = 'Indiquez l\'identifiant du membre.'
    return
  }
  if (!confirm(
    `Retirer « ${c.badge.libelle} » à ce membre ?\n\nLe membre n'en sera pas informé ; le retrait est tracé dans l'audit.`,
  )) return
  try {
    await retirerBadge(c.badge.id, c.utilisateur_id.trim())
    await rafraichir()
    notifier('Badge retiré (tracé dans l\'audit, sans notification)')
    cible.value = { badge: null, utilisateur_id: '', motif: '' }
  } catch (e) { signaler(e) }
}
</script>

<template>
  <div class="p-6 space-y-6">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Engagement, Badges & succès</h1>
      <div class="flex flex-wrap gap-2">
        <NuxtLink to="/admin/engagement/regles" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-sliders" /> Barème
        </NuxtLink>
        <button class="btn btn-sm btn-primary" @click="formulaire = !formulaire">
          <font-awesome-icon :icon="formulaire ? 'fa-solid fa-xmark' : 'fa-solid fa-plus'" />
          {{ formulaire ? 'Annuler' : 'Nouveau badge' }}
        </button>
      </div>
    </div>

    <div class="alert alert-info py-2 text-sm">
      <font-awesome-icon icon="fa-solid fa-circle-info" />
      <span>
        Un badge désactivé disparaît du catalogue « à débloquer » mais
        <strong>reste chez les membres qui l'ont obtenu</strong>. La description est le texte
        que le membre lit : formulez-y la condition en langage clair.
      </span>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>
    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else>
      <!-- ─── Création ─── -->
      <div v-if="formulaire" class="card bg-base-200">
        <div class="card-body gap-3">
          <h2 class="font-semibold">Créer un badge</h2>

          <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
            <label class="form-control">
              <span class="label-text text-xs">Code (immuable ensuite)</span>
              <input v-model="nouveau.code" class="input input-sm input-bordered font-mono" placeholder="ex. mentor_confirme">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Libellé</span>
              <input v-model="nouveau.libelle" class="input input-sm input-bordered" placeholder="ex. Mentor confirmé">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Ordre d'affichage</span>
              <input v-model.number="nouveau.ordre" type="number" class="input input-sm input-bordered">
            </label>
            <label class="form-control md:col-span-3">
              <span class="label-text text-xs">Description, la condition en langage clair, lue par le membre</span>
              <textarea v-model="nouveau.description" rows="2" class="textarea textarea-sm textarea-bordered" placeholder="ex. 10 contributions validées." />
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Couleur</span>
              <select v-model="nouveau.couleur" class="select select-sm select-bordered">
                <option value="">par défaut</option>
                <option v-for="c in COULEURS" :key="c" :value="c">{{ c }}</option>
              </select>
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Icône (FontAwesome)</span>
              <input v-model="nouveau.icone" class="input input-sm input-bordered font-mono" placeholder="ex. handshake">
            </label>
            <label class="label cursor-pointer justify-start gap-2">
              <input v-model="nouveau.manuel" type="checkbox" class="checkbox checkbox-sm">
              <span class="label-text text-xs">Badge manuel (distinction éditoriale)</span>
            </label>
          </div>

          <!-- Condition : champs adaptés au type retenu -->
          <div v-if="!nouveau.manuel" class="grid grid-cols-1 gap-3 md:grid-cols-3">
            <label class="form-control">
              <span class="label-text text-xs">Condition</span>
              <select v-model="nouveau.type_condition" class="select select-sm select-bordered">
                <option v-for="c in CONDITIONS" :key="c.valeur" :value="c.valeur">{{ c.libelle }}</option>
              </select>
            </label>

            <label v-if="champsCondition.includes('action')" class="form-control">
              <span class="label-text text-xs">Action à compter</span>
              <select v-model="nouveau.parametre_action" class="select select-sm select-bordered">
                <option value="">choisir</option>
                <option v-for="a in actions" :key="a" :value="a">{{ a }}</option>
              </select>
            </label>

            <label v-if="champsCondition.includes('categorie')" class="form-control">
              <span class="label-text text-xs">Catégorie visée</span>
              <select v-model="nouveau.parametre_categorie_id" class="select select-sm select-bordered">
                <option value="">choisir</option>
                <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.libelle }}</option>
              </select>
            </label>

            <label v-if="champsCondition.includes('niveau')" class="form-control">
              <span class="label-text text-xs">Niveau visé</span>
              <select v-model="nouveau.parametre_niveau_code" class="select select-sm select-bordered">
                <option value="">choisir</option>
                <option v-for="n in niveaux" :key="n.id" :value="n.code">{{ n.libelle }}</option>
              </select>
            </label>

            <label v-if="champsCondition.includes('seuil')" class="form-control">
              <span class="label-text text-xs">Seuil à atteindre</span>
              <input v-model.number="nouveau.seuil" type="number" class="input input-sm input-bordered">
            </label>
          </div>

          <div class="card-actions justify-end">
            <button class="btn btn-sm btn-success" @click="ajouter">Créer</button>
          </div>
        </div>
      </div>

      <!-- ─── Tableau ─── -->
      <div class="overflow-x-auto">
        <table class="table table-zebra table-sm">
          <thead>
            <tr>
              <th>Ordre</th><th>Code</th><th>Libellé</th><th>Description</th>
              <th>Condition</th><th>Paramètres</th><th>Apparence</th>
              <th>Détenteurs</th><th>Actif</th><th />
            </tr>
          </thead>
          <tbody>
            <tr v-for="b in badges" :key="b.id">
              <td><input v-model.number="b.ordre" type="number" class="input input-sm input-bordered w-16"></td>
              <td class="font-mono text-xs">{{ b.code }}</td>
              <td><input v-model="b.libelle" class="input input-sm input-bordered w-40"></td>
              <td><textarea v-model="b.description" rows="2" class="textarea textarea-sm textarea-bordered w-56" /></td>
              <td class="text-xs">
                <label class="label cursor-pointer justify-start gap-1 p-0">
                  <input v-model="b.manuel" type="checkbox" class="checkbox checkbox-xs">
                  <span>manuel</span>
                </label>
                <select v-if="!b.manuel" v-model="b.type_condition" class="select select-sm select-bordered mt-1 w-40">
                  <option v-for="c in CONDITIONS" :key="c.valeur" :value="c.valeur">{{ c.libelle }}</option>
                </select>
                <span v-else class="opacity-60">{{ libelleCondition(b) }}</span>
              </td>
              <td class="space-y-1">
                <template v-if="!b.manuel">
                  <select
                    v-if="champsPour(b.type_condition).includes('action')"
                    v-model="b.parametre_action"
                    class="select select-sm select-bordered w-44"
                  >
                    <option :value="null">action</option>
                    <option v-for="a in actions" :key="a" :value="a">{{ a }}</option>
                  </select>
                  <select
                    v-if="champsPour(b.type_condition).includes('categorie')"
                    v-model="b.parametre_categorie_id"
                    class="select select-sm select-bordered w-44"
                  >
                    <option :value="null">catégorie</option>
                    <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.libelle }}</option>
                  </select>
                  <select
                    v-if="champsPour(b.type_condition).includes('niveau')"
                    v-model="b.parametre_niveau_code"
                    class="select select-sm select-bordered w-44"
                  >
                    <option :value="null">niveau</option>
                    <option v-for="n in niveaux" :key="n.id" :value="n.code">{{ n.libelle }}</option>
                  </select>
                  <input
                    v-if="champsPour(b.type_condition).includes('seuil')"
                    v-model.number="b.seuil"
                    type="number"
                    class="input input-sm input-bordered w-24"
                    placeholder="seuil"
                  >
                </template>
                <span v-else class="text-xs opacity-60">-</span>
              </td>
              <td class="space-y-1">
                <select v-model="b.couleur" class="select select-sm select-bordered w-28">
                  <option :value="null">-</option>
                  <option v-for="c in COULEURS" :key="c" :value="c">{{ c }}</option>
                </select>
                <input v-model="b.icone" class="input input-sm input-bordered w-28 font-mono" placeholder="icône">
              </td>
              <td class="text-center text-xs">{{ b.nombre_detenteurs }}</td>
              <td><input v-model="b.actif" type="checkbox" class="toggle toggle-sm"></td>
              <td class="flex flex-col gap-1">
                <button class="btn btn-xs btn-primary" @click="enregistrer(b)">Enregistrer</button>
                <button class="btn btn-xs btn-outline" @click="cible = { badge: b, utilisateur_id: '', motif: '' }">
                  Attribuer…
                </button>
                <button
                  class="btn btn-xs btn-error btn-outline"
                  :disabled="b.nombre_detenteurs > 0"
                  :title="b.nombre_detenteurs > 0 ? 'Badge détenu : désactivez-le plutôt' : 'Supprimer'"
                  @click="supprimer(b)"
                >
                  <font-awesome-icon icon="fa-solid fa-trash" />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- ─── Attribution / retrait manuels ─── -->
      <div v-if="cible.badge" class="card bg-base-200">
        <div class="card-body gap-3">
          <h2 class="font-semibold">
            Attribution manuelle : « {{ cible.badge.libelle }} »
          </h2>
          <div class="flex flex-wrap items-end gap-3">
            <label class="form-control">
              <span class="label-text text-xs">Identifiant du membre</span>
              <input v-model="cible.utilisateur_id" class="input input-sm input-bordered w-80 font-mono" placeholder="UUID">
            </label>
            <label class="form-control flex-1">
              <span class="label-text text-xs">Motif (tracé dans l'audit)</span>
              <input v-model="cible.motif" class="input input-sm input-bordered w-full" placeholder="Raison de la distinction">
            </label>
            <button class="btn btn-sm btn-success" @click="soumettreAttribution">Attribuer</button>
            <button class="btn btn-sm btn-error btn-outline" @click="soumettreRetrait">Retirer</button>
            <button class="btn btn-sm btn-ghost" @click="cible = { badge: null, utilisateur_id: '', motif: '' }">
              Fermer
            </button>
          </div>
          <p class="text-xs opacity-70">
            L'attribution notifie le membre ; le <strong>retrait ne le notifie pas</strong> : c'est
            un geste de correction, tracé dans la piste d'audit.
          </p>
        </div>
      </div>
    </template>
  </div>
</template>
